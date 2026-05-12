pkgdatadir = get_option('prefix') / get_option('datadir') / meson.project_name()
gnome = import('gnome')

blueprints = custom_target(
  'blueprints',
  input: files(
    'ui/shortcuts-dialog.blp',
    'ui/color-selector.blp',
    'ui/color-box.blp',
    'ui/window.blp',
  ),
  output: '.',
  command: [
    find_program('blueprint-compiler'),
    'batch-compile',
    '--minify', '@OUTPUT@',
    '@CURRENT_SOURCE_DIR@/ui',
    '@INPUT@',
  ],
)

gnome.compile_resources(
  'piccolo',
  'ui/piccolo.gresource.xml',
  source_dir: ['./ui', '../data/icons/hicolor/scalable/actions/'],
  gresource_bundle: true,
  dependencies: blueprints,
  install: true,
  install_dir: pkgdatadir,
)

cargo_bin = find_program('cargo')
cargo_opt = ['--manifest-path', meson.project_source_root() / 'Cargo.toml']
cargo_opt += ['--target-dir', meson.project_build_root() / 'src']
cargo_env = {
  'CARGO_HOME': meson.project_build_root() / 'cargo-home',
  'APP_ID': app_id,
  'VERSION': meson.project_version(),
  'GETTEXT_PACKAGE': 'piccolo',
  'LOCALEDIR': get_option('prefix') / get_option('localedir'),
  'PKGDATADIR': pkgdatadir,
}

if get_option('buildtype') == 'release'
  cargo_opt += ['--release']
  rust_target = 'release'
else
  rust_target = 'debug'
endif

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  env: cargo_env,
  command: [
    cargo_bin,
    'build',
    cargo_opt,
    '&&',
    'cp',
    'src' / rust_target / meson.project_name(),
    '@OUTPUT@',
  ],
)
