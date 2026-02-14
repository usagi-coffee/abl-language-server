use crate::analysis::buffers::collect_buffer_mappings;
use crate::analysis::definitions::{
    AblDefinitionSite, PreprocessorDefineSite, collect_definition_sites,
    collect_function_definition_sites, collect_global_preprocessor_define_sites,
    collect_local_table_field_sites, collect_preprocessor_define_sites,
};
use crate::analysis::includes::{
    collect_include_sites_from_tree, include_site_matches_file_offset, resolve_include_site_path,
};
use crate::analysis::schema::normalize_lookup_key;
use crate::analysis::schema_lookup::pick_single_location;
use crate::analysis::scopes::containing_scope;
use crate::backend::Backend;
use std::collections::HashMap;
use std::path::PathBuf;
use tower_lsp::lsp_types::{Location, Position, Range, Url};
use tree_sitter::Node;

pub async fn resolve_include_directive_location(
    backend: &Backend,
    uri: &Url,
    text: &str,
    root: Node<'_>,
    offset: usize,
) -> Option<Location> {
    let include_sites = collect_include_sites_from_tree(root, text.as_bytes());
    let target = include_sites
        .into_iter()
        .find(|site| include_site_matches_file_offset(site, offset))?;
    let target_key = (target.start_offset, target.file_start_offset);

    let current_path = uri.to_file_path().ok()?;
    let mut available_define_sites = Vec::new();
    collect_preprocessor_define_sites(root, text.as_bytes(), &mut available_define_sites);

    let include_sites = collect_include_sites_from_tree(root, text.as_bytes());
    let mut include_path: Option<PathBuf> = None;

    for include in include_sites {
        if include.start_offset > target_key.0 {
            break;
        }

        let include_path_value = resolve_include_site_path(&include, &available_define_sites);
        let Some(resolved_path) = backend
            .resolve_include_path_for(&current_path, &include_path_value)
            .await
        else {
            continue;
        };

        if (include.start_offset, include.file_start_offset) == target_key {
            include_path = Some(resolved_path);
            break;
        }

        let Some((include_text, include_tree)) =
            backend.get_cached_include_parse(&resolved_path).await
        else {
            continue;
        };
        let mut include_global_defines = Vec::new();
        collect_global_preprocessor_define_sites(
            include_tree.root_node(),
            include_text.as_bytes(),
            &mut include_global_defines,
        );
        for mut define in include_global_defines {
            define.start_byte = include.start_offset;
            available_define_sites.push(define);
        }
    }

    let include_path = include_path?;
    let include_uri = Url::from_file_path(include_path).ok()?;

    Some(Location {
        uri: include_uri,
        range: Range::new(Position::new(0, 0), Position::new(0, 0)),
    })
}

pub fn resolve_buffer_alias_table_location(
    backend: &Backend,
    uri: &Url,
    root: Node<'_>,
    src: &[u8],
    symbol_upper: &str,
    offset: usize,
) -> Option<Location> {
    let mut buffer_mappings = Vec::new();
    collect_buffer_mappings(root, src, &mut buffer_mappings);
    let mut buffer_before: Option<(usize, String)> = None;
    let mut buffer_after: Option<(usize, String)> = None;
    for mapping in buffer_mappings {
        if !mapping.alias.eq_ignore_ascii_case(symbol_upper) {
            continue;
        }
        let table_key = normalize_lookup_key(&mapping.table, false);
        if mapping.start_byte <= offset {
            let should_take = buffer_before
                .as_ref()
                .map(|(start, _)| mapping.start_byte > *start)
                .unwrap_or(true);
            if should_take {
                buffer_before = Some((mapping.start_byte, table_key));
            }
        } else {
            let should_take = buffer_after
                .as_ref()
                .map(|(start, _)| mapping.start_byte < *start)
                .unwrap_or(true);
            if should_take {
                buffer_after = Some((mapping.start_byte, table_key));
            }
        }
    }

    if let Some((_, table_key)) = buffer_before.or(buffer_after) {
        let mut local_sites = Vec::new();
        collect_definition_sites(root, src, &mut local_sites);

        let mut best_before: Option<(usize, Range)> = None;
        let mut best_after: Option<(usize, Range)> = None;
        for site in local_sites {
            if !site.label.eq_ignore_ascii_case(&table_key) {
                continue;
            }
            if site.start_byte <= offset {
                let should_take = best_before
                    .as_ref()
                    .map(|(start, _)| site.start_byte > *start)
                    .unwrap_or(true);
                if should_take {
                    best_before = Some((site.start_byte, site.range));
                }
            } else {
                let should_take = best_after
                    .as_ref()
                    .map(|(start, _)| site.start_byte < *start)
                    .unwrap_or(true);
                if should_take {
                    best_after = Some((site.start_byte, site.range));
                }
            }
        }
        if let Some((_, range)) = best_before.or(best_after) {
            return Some(Location {
                uri: uri.clone(),
                range,
            });
        }

        if let Some(locations) = backend.db_table_definitions.get(&table_key)
            && let Some(location) = pick_single_location(locations.value())
        {
            return Some(location);
        }
    }

    None
}

pub fn resolve_local_definition_location(
    uri: &Url,
    root: Node<'_>,
    src: &[u8],
    symbol: &str,
    offset: usize,
) -> Option<Location> {
    let mut sites = Vec::new();
    collect_definition_sites(root, src, &mut sites);
    collect_local_table_field_sites(root, src, &mut sites);

    let mut best_before: Option<(usize, Range)> = None;
    let mut best_after: Option<(usize, Range)> = None;

    for site in sites {
        if !site.label.eq_ignore_ascii_case(symbol) {
            continue;
        }

        if site.start_byte <= offset {
            let should_take = best_before
                .as_ref()
                .map(|(start, _)| site.start_byte > *start)
                .unwrap_or(true);
            if should_take {
                best_before = Some((site.start_byte, site.range));
            }
        } else {
            let should_take = best_after
                .as_ref()
                .map(|(start, _)| site.start_byte < *start)
                .unwrap_or(true);
            if should_take {
                best_after = Some((site.start_byte, site.range));
            }
        }
    }

    best_before.or(best_after).map(|(_, range)| Location {
        uri: uri.clone(),
        range,
    })
}

pub async fn resolve_include_function_location(
    backend: &Backend,
    uri: &Url,
    text: &str,
    root: Node<'_>,
    symbol: &str,
    offset: usize,
) -> Option<Location> {
    let scope = containing_scope(root, offset)?;
    let current_path = uri.to_file_path().ok()?;
    let include_sites = collect_include_sites_from_tree(root, text.as_bytes());
    let mut available_define_sites = Vec::new();
    collect_preprocessor_define_sites(root, text.as_bytes(), &mut available_define_sites);

    let mut parsed_include_functions: HashMap<PathBuf, Vec<AblDefinitionSite>> = HashMap::new();
    let mut include_before: Option<(usize, Location)> = None;
    let mut include_after: Option<(usize, Location)> = None;

    for include in include_sites {
        if include.start_offset < scope.start || include.start_offset > scope.end {
            continue;
        }

        let include_path_value = resolve_include_site_path(&include, &available_define_sites);
        let Some(include_path) = backend
            .resolve_include_path_for(&current_path, &include_path_value)
            .await
        else {
            continue;
        };

        if !parsed_include_functions.contains_key(&include_path) {
            let Some((include_text, include_tree)) =
                backend.get_cached_include_parse(&include_path).await
            else {
                continue;
            };

            let mut function_sites = Vec::new();
            collect_function_definition_sites(
                include_tree.root_node(),
                include_text.as_bytes(),
                &mut function_sites,
            );
            parsed_include_functions.insert(include_path.clone(), function_sites);
        }

        let Some(function_sites) = parsed_include_functions.get(&include_path) else {
            continue;
        };

        let Some(include_uri) = Url::from_file_path(&include_path).ok() else {
            continue;
        };

        for site in function_sites {
            if !site.label.eq_ignore_ascii_case(symbol) {
                continue;
            }

            let location = Location {
                uri: include_uri.clone(),
                range: site.range,
            };

            if include.start_offset <= offset {
                let should_take = include_before
                    .as_ref()
                    .map(|(site_offset, _)| include.start_offset > *site_offset)
                    .unwrap_or(true);
                if should_take {
                    include_before = Some((include.start_offset, location));
                }
            } else {
                let should_take = include_after
                    .as_ref()
                    .map(|(site_offset, _)| include.start_offset < *site_offset)
                    .unwrap_or(true);
                if should_take {
                    include_after = Some((include.start_offset, location));
                }
            }
        }

        if let Some((include_text, include_tree)) =
            backend.get_cached_include_parse(&include_path).await
        {
            let mut include_global_defines = Vec::new();
            collect_global_preprocessor_define_sites(
                include_tree.root_node(),
                include_text.as_bytes(),
                &mut include_global_defines,
            );
            for mut define in include_global_defines {
                define.start_byte = include.start_offset;
                available_define_sites.push(define);
            }
        }
    }

    include_before
        .or(include_after)
        .map(|(_, location)| location)
}

pub async fn resolve_include_definition_location(
    backend: &Backend,
    uri: &Url,
    text: &str,
    root: Node<'_>,
    symbol: &str,
    offset: usize,
) -> Option<Location> {
    let scope = containing_scope(root, offset)?;
    let current_path = uri.to_file_path().ok()?;
    let include_sites = collect_include_sites_from_tree(root, text.as_bytes());
    let mut available_define_sites = Vec::new();
    collect_preprocessor_define_sites(root, text.as_bytes(), &mut available_define_sites);

    let mut parsed_include_defs: HashMap<PathBuf, Vec<AblDefinitionSite>> = HashMap::new();
    let mut include_before: Option<(usize, Location)> = None;
    let mut include_after: Option<(usize, Location)> = None;

    for include in include_sites {
        if include.start_offset < scope.start || include.start_offset > scope.end {
            continue;
        }

        let include_path_value = resolve_include_site_path(&include, &available_define_sites);
        let Some(include_path) = backend
            .resolve_include_path_for(&current_path, &include_path_value)
            .await
        else {
            continue;
        };

        if !parsed_include_defs.contains_key(&include_path) {
            let Some((include_text, include_tree)) =
                backend.get_cached_include_parse(&include_path).await
            else {
                continue;
            };

            let mut sites = Vec::new();
            collect_definition_sites(
                include_tree.root_node(),
                include_text.as_bytes(),
                &mut sites,
            );
            collect_local_table_field_sites(
                include_tree.root_node(),
                include_text.as_bytes(),
                &mut sites,
            );
            parsed_include_defs.insert(include_path.clone(), sites);

            let mut include_global_defines = Vec::new();
            collect_global_preprocessor_define_sites(
                include_tree.root_node(),
                include_text.as_bytes(),
                &mut include_global_defines,
            );
            for mut define in include_global_defines {
                define.start_byte = include.start_offset;
                available_define_sites.push(define);
            }
        }

        let Some(def_sites) = parsed_include_defs.get(&include_path) else {
            continue;
        };
        let Some(include_uri) = Url::from_file_path(&include_path).ok() else {
            continue;
        };

        for site in def_sites {
            if !site.label.eq_ignore_ascii_case(symbol) {
                continue;
            }

            let location = Location {
                uri: include_uri.clone(),
                range: site.range,
            };

            if include.start_offset <= offset {
                let should_take = include_before
                    .as_ref()
                    .map(|(site_offset, _)| include.start_offset > *site_offset)
                    .unwrap_or(true);
                if should_take {
                    include_before = Some((include.start_offset, location));
                }
            } else {
                let should_take = include_after
                    .as_ref()
                    .map(|(site_offset, _)| include.start_offset < *site_offset)
                    .unwrap_or(true);
                if should_take {
                    include_after = Some((include.start_offset, location));
                }
            }
        }
    }

    include_before
        .or(include_after)
        .map(|(_, location)| location)
}

pub struct PreprocessorDefineMatch {
    pub name: String,
    pub value: Option<String>,
    pub is_global: bool,
    pub location: Location,
}

pub async fn resolve_preprocessor_define_match(
    backend: &Backend,
    uri: &Url,
    text: &str,
    root: Node<'_>,
    symbol: &str,
    offset: usize,
) -> Option<PreprocessorDefineMatch> {
    let mut local_sites = Vec::new();
    collect_preprocessor_define_sites(root, text.as_bytes(), &mut local_sites);
    if let Some((site, range)) = pick_best_preprocessor_site(&local_sites, symbol, offset) {
        return Some(PreprocessorDefineMatch {
            name: site.label.clone(),
            value: site.value.clone(),
            is_global: site.is_global,
            location: Location {
                uri: uri.clone(),
                range,
            },
        });
    }

    let scope = containing_scope(root, offset)?;
    let current_path = uri.to_file_path().ok()?;
    let include_sites = collect_include_sites_from_tree(root, text.as_bytes());
    let mut available_define_sites = Vec::new();
    collect_preprocessor_define_sites(root, text.as_bytes(), &mut available_define_sites);

    let mut parsed_include_defines: HashMap<PathBuf, Vec<PreprocessorDefineSite>> = HashMap::new();
    let mut include_before: Option<(usize, PreprocessorDefineMatch)> = None;
    let mut include_after: Option<(usize, PreprocessorDefineMatch)> = None;

    for include in include_sites {
        if include.start_offset < scope.start || include.start_offset > scope.end {
            continue;
        }
        let include_path_value = resolve_include_site_path(&include, &available_define_sites);
        let Some(include_path) = backend
            .resolve_include_path_for(&current_path, &include_path_value)
            .await
        else {
            continue;
        };

        if !parsed_include_defines.contains_key(&include_path) {
            let Some((include_text, include_tree)) =
                backend.get_cached_include_parse(&include_path).await
            else {
                continue;
            };
            let mut define_sites = Vec::new();
            collect_global_preprocessor_define_sites(
                include_tree.root_node(),
                include_text.as_bytes(),
                &mut define_sites,
            );
            parsed_include_defines.insert(include_path.clone(), define_sites);
        }

        let Some(define_sites) = parsed_include_defines.get(&include_path) else {
            continue;
        };
        let Some(site) = define_sites
            .iter()
            .find(|s| s.label.eq_ignore_ascii_case(symbol))
        else {
            continue;
        };
        let Some(include_uri) = Url::from_file_path(&include_path).ok() else {
            continue;
        };

        let matched = PreprocessorDefineMatch {
            name: site.label.clone(),
            value: site.value.clone(),
            is_global: true,
            location: Location {
                uri: include_uri,
                range: site.range,
            },
        };
        if include.start_offset <= offset {
            let should_take = include_before
                .as_ref()
                .map(|(site_offset, _)| include.start_offset > *site_offset)
                .unwrap_or(true);
            if should_take {
                include_before = Some((include.start_offset, matched));
            }
        } else {
            let should_take = include_after
                .as_ref()
                .map(|(site_offset, _)| include.start_offset < *site_offset)
                .unwrap_or(true);
            if should_take {
                include_after = Some((include.start_offset, matched));
            }
        }

        if let Some((include_text, include_tree)) =
            backend.get_cached_include_parse(&include_path).await
        {
            let mut include_global_defines = Vec::new();
            collect_global_preprocessor_define_sites(
                include_tree.root_node(),
                include_text.as_bytes(),
                &mut include_global_defines,
            );
            for mut define in include_global_defines {
                define.start_byte = include.start_offset;
                available_define_sites.push(define);
            }
        }
    }

    include_before.or(include_after).map(|(_, m)| m)
}

fn pick_best_preprocessor_site<'a>(
    sites: &'a [PreprocessorDefineSite],
    symbol: &str,
    offset: usize,
) -> Option<(&'a PreprocessorDefineSite, Range)> {
    let mut best_before: Option<(&PreprocessorDefineSite, Range)> = None;
    let mut best_after: Option<(&PreprocessorDefineSite, Range)> = None;
    for site in sites {
        if !site.label.eq_ignore_ascii_case(symbol) {
            continue;
        }
        if site.start_byte <= offset {
            let should_take = best_before
                .as_ref()
                .map(|(s, _)| site.start_byte > s.start_byte)
                .unwrap_or(true);
            if should_take {
                best_before = Some((site, site.range));
            }
        } else {
            let should_take = best_after
                .as_ref()
                .map(|(s, _)| site.start_byte < s.start_byte)
                .unwrap_or(true);
            if should_take {
                best_after = Some((site, site.range));
            }
        }
    }
    best_before.or(best_after)
}
