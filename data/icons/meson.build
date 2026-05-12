application_id = 'art.FatDawlf.Piccolo'

scalable_dir = 'hicolor' / 'scalable' / 'apps'
install_data(
  scalable_dir / ('@0@.svg').format(application_id),
  install_dir: get_option('datadir') / 'icons' / scalable_dir
)

symbolic_dir = 'hicolor' / 'symbolic' / 'apps'
install_data(
  symbolic_dir / ('@0@-symbolic.svg').format(application_id),
  install_dir: get_option('datadir') / 'icons' / symbolic_dir
)

actions_dir = 'hicolor' / 'scalable' / 'actions'
install_data(
  actions_dir / 'sidebar-show-symbolic.svg',
  actions_dir / 'color-picker-symbolic.svg',
  actions_dir / 'copy-symbolic.svg',
  install_dir: get_option('datadir') / 'icons' / actions_dir
  )
