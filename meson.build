project(
  'piccolo',
  'rust',
  version: '0.2.0',
  meson_version: '>= 1.0.0',
  default_options: [
    'warning_level=2',
    'werror=false',
  ],
)

app_id = 'art.fatdawlf.Piccolo@0@'.format(get_option('app_id_suffix'))
bin_name = 'piccolo'

i18n = import('i18n')
gnome = import('gnome')

subdir('data')
subdir('src')
subdir('po')

gnome.post_install(
  glib_compile_schemas: true,
  gtk_update_icon_cache: true,
  update_desktop_database: true,
)
