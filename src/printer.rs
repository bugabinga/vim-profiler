use crate::common::*;

#[derive(Debug)]
pub struct Printer {
  reverse: bool,
  count:   Option<usize>,
  prec:    Option<usize>,
}

impl Printer {
  pub fn new(reverse: bool, count: Option<usize>, prec: Option<usize>) -> Self {
    Self {
      reverse,
      count,
      prec,
    }
  }

  pub fn summary(&self, plugins: &[Plugin]) {
    let mut plugins = plugins.to_owned();

    plugins.truncate(self.count.unwrap_or(10_usize));

    let header = format!(
      "Top {} {} (n)vim plugins.",
      plugins.len(),
      if self.reverse { "fastest" } else { "slowest" }
    );

    println!("{}", header);
    println!("{}", utils::repeat("=", header.len()));

    for (i, plugin) in plugins.iter().enumerate() {
      println!(
        "{} {} {:10}",
        format!("{:<1$}", i + 1, plugins.len().to_string().len() + 2),
        format!("{:1$}", plugin.name, &plugins.len_largest()),
        format!("{:.1$}", plugin.average(), self.prec.unwrap_or(2_usize))
      );
    }

    println!("{}", utils::repeat("=", header.len()));
  }
}
