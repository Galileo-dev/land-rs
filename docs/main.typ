#import "@preview/glossarium:0.5.1": register-glossary, make-glossary, print-glossary, gls, glspl
#import "@preview/codly:1.2.0": *
#import "styles/styles.typ": *
#import "styles/abbreviations.typ": abbreviations-page
#import "styles/frontpage.typ": frontpage



#let abbreviations = (
  (
    key: "ul",
    short: "UL",
    long: "University of Limerick",
  ),
  (
    key: "cpu",
    short: "CPU",
    long: "Central Processing Unit",
  ),
  (
    key: "gpu",
    short: "GPU",
    long: "Graphics Processing Unit",
  ),
)


#show: make-glossary
#register-glossary(abbreviations)
#show: codly-init


// Code blocks
#codly(
  languages: (
    rust: (
      name: "Rust",
      color: rgb("#CE412B"),
    ),
    // NOTE: Hacky, but 'fs' doesn't syntax highlight
    fsi: (
      name: "F#",
      color: rgb("#6a0dad"),
    ),
  ),
)

// Use front page stylings
#show: front-page-style

#show: frontpage(
  title: [Convex Optimization vs. RL for Rocket Landing],
  subtitle: "Investigating the performance of convex optimization vs reinforcement learning for landing autonomous rockets",
  author: "Fionn Barrett",
  degree: "Bachelor of Immersive Software Engineering",
  faculty: "Faculty of Science and Engineering",
  department: "Department of Computer Science",
  date: datetime(year: 2025, month: 2, day: 18),
)

#abbreviations-page(abbreviations)


// Use Main page stylings
#show: main-page-style

// chapters
