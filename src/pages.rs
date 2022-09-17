// create a struct to hold information about page links
pub struct Page {
  pub name: &'static str,
  pub link: &'static str,
}

// create a vector of page structs that will build the navigation bar
pub const PAGES: [Page; 2] = [
  Page {
    name: "Home",
    link: "/"
  },
  Page {
    name: "Gallery",
    link: "/gallery"
  }
];