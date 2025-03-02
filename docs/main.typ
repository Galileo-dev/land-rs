#import "@preview/glossarium:0.5.3": make-glossary, register-glossary, print-glossary, gls, glspl
#import "@preview/codly:1.2.0": *
#import "@preview/codly-languages:0.1.1": *
#import "styles/styles.typ": *
#import "styles/abbreviations.typ": abbreviations-page
#import "styles/frontpage.typ": frontpage
#import "styles/components.typ": *
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

#set text(font: "Inter", size: 11pt, fill: rgb(51, 51, 51))
#show raw: set text(font: "SF Mono", size: 10pt, fill: rgb(80, 80, 80))
#set par(leading: 0.7em)

// Code block styling
#show raw.where(block: true): block => {
  set block(
    fill: rgb(250, 250, 250),
    radius: 4pt,
    inset: 10pt,
    width: 100%,
  )
  block
}

// Table styling
#show table: table => {
  clean-table(
    table.headers,
    ..table.rows,
  )
}

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

// Use Main page stylings
#show: main-page-style

// Table of contents
#outline(
  title: [Contents],
  indent: 1em,
)
#pagebreak()

#include "./chapters/introduction.typ"
#pagebreak()

#include "./chapters/literature_review.typ"
#pagebreak()

#include "./chapters/methodology.typ"
#pagebreak()

#include "./chapters/progress_update.typ"
#pagebreak()

#include "./chapters/simulation.typ"
#pagebreak()

#include "./chapters/convex_approch.typ"
#pagebreak()

= References
#bibliography
