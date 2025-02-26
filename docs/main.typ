#import "@preview/glossarium:0.5.3": make-glossary, register-glossary, print-glossary, gls, glspl
#import "@preview/codly:1.2.0": *
#import "@preview/codly-languages:0.1.1": *
#import "styles/styles.typ": *
#import "styles/abbreviations.typ": abbreviations-page
#import "styles/frontpage.typ": frontpage
#import "utils/symbols.typ": *

#show: make-glossary
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
#register-glossary(abbreviations)

#show: codly-init.with()

#set text(font: "Inter Nerd Font Propo", size: 11pt)

#show raw: set text(font: "Monaspace Argon", size: 9pt)

// Define bibliography
#let bibliography = bibliography("references.bib", style: "ieee")

// Use front page stylings
#show: front-page-style

#frontpage(
  title: [Convex Optimization vs. RL for Rocket Landing],
  subtitle: "Investigating the performance of convex optimization vs reinforcement learning for landing autonomous rockets",
  author: "Fionn Barrett",
  degree: "Bachelor of Immersive Software Engineering",
  faculty: "Faculty of Science and Engineering",
  department: "Department of Computer Science",
  date: datetime(year: 2025, month: 2, day: 18),
)

// TODO: Fix abbreviations page (currently doesn't display anything)
// #show: abbreviations-page(abbreviations)

// Use Main page stylings
#show: main-page-style

= Introduction <chp:introduction>
#include "./chapters/introduction.typ"
#pagebreak()

= Literature Review <chp:literature_review>
#include "./chapters/literature_review.typ"
#pagebreak()

= Methodology <chp:methodology>
#include "./chapters/methodology.typ"
#pagebreak()

= Progress Update <chp:progress_update>
#include "./chapters/progress_update.typ"
#pagebreak()

= References
#bibliography
