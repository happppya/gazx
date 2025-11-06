use indicatif::ProgressStyle;

pub fn style_build_population() -> ProgressStyle {
  let style = ProgressStyle::with_template("[{elapsed_precise}] {msg} {bar:40.cyan/blue}").unwrap().progress_chars("##-");
  return style;
}

pub fn style_print_population() -> ProgressStyle {
  let style = ProgressStyle::with_template("[{elapsed_precise}] {msg} {bar:40.cyan/blue}").unwrap().progress_chars("##-");
  return style;
}