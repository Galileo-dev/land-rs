#import "@preview/glossarium:0.5.3": make-glossary, register-glossary, print-glossary, gls, glspl
#import "@preview/codly:1.2.0": *
#import "@preview/codly-languages:0.1.1": *
#import "styles/styles.typ": *
#import "styles/abbreviations.typ": abbreviations-page
#import "styles/frontpage.typ": frontpage
#import "styles/components.typ": *
#import "utils/symbols.typ": *
#import "@preview/cheq:0.2.2": checklist

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
  (
    key: "twr",
    short: "TWR",
    long: "Thrust-to-Weight Ratio",
  ),
  (
    key: "rl",
    short: "RL",
    long: "Reinforcement Learning",
  ),
  (
    key: "gfold",
    short: "G-FOLD",
    long: "Guidance for Fuel Optimal Large Diverts",
  ),
  (
    key: "pdg",
    short: "PDG",
    long: "Powered Descent Guidance",
  ),
  (
    key: "apdg",
    short: "APDG",
    long: "Atmospheric Powered Descent Guidance",
  ),
  (
    key: "ecs",
    short: "ECS",
    long: "Entity Component System",
  ),
  (
    key: "agc",
    short: "AGC",
    long: "Apollo Guidance Computer",
  ),
  (
    key: "socp",
    short: "SOCP",
    long: "Second Order Cone Programming",
  ),
  (
    key: "soc",
    short: "SOC",
    long: "Second Order Cone",
  ),
  (
    key: "rlv",
    short: "RLV",
    long: "Reusable Launch Vehicle",
  ),
  (
    key: "6dof",
    short: "6DOF",
    long: "Six Degrees of Freedom",
  ),
  (
    key: "lp",
    short: "LP",
    long: "Linear Programming",
  ),
  (
    key: "fyp",
    short: "FYP",
    long: "Final Year Project",
  ),
  (
    key: "sc",
    short: "SCvx",
    long: "Successive Convexification",
  ),
  (
    key: "ipm",
    short: "IPM",
    long: "Interior Point Method",
  ),
)


#register-glossary(abbreviations)

#show: codly-init
#codly(zebra-fill: none, languages: codly-languages)


#set text(font: "Times New Roman", size: 11pt, fill: rgb(51, 51, 51))
#show raw: set text(font: "Monaspace Argon", size: 10pt, fill: rgb(80, 80, 80))
#set par(leading: 0.7em)

// Table styling
#show table: table => {
  clean-table(
    table.headers,
    ..table.rows,
  )
}

// Define bibliography
#let bibliography = bibliography("references.bib", style: "ieee")

// Define the checklist style
#show: checklist.with(fill: luma(95%), stroke: blue, radius: .2em)

// Use front page stylings
#show: front-page-style
#frontpage(
  title: [Convex Optimization vs. RL for Rocket Landing],
  subtitle: "Investigating the performance of convex optimization vs reinforcement learning for landing autonomous rockets",
  author: "Fionn Barrett",
  degree: "Bachelor of Immersive Software Engineering",
  faculty: "Faculty of Science and Engineering",
  department: "Department of Computer Science",
  date: datetime(year: 2025, month: 5, day: 15),
)

// Use Main page stylings
#show: main-page-style


// Table of contents
#outline(
  title: [Contents],
  indent: 1em,
)
#pagebreak()

// List of figures
#outline(
  title: [List of Figures],
  target: figure.where(kind: image),
)
#pagebreak()

// List of tables
// #outline(
//   title: [List of Tables],
//   target: figure.where(kind: table),
// )
// #pagebreak()

// Abbreviations
#abbreviations-page(abbreviations)
#pagebreak()

// Abstract
#include "./chapters/abstract.typ"
#pagebreak()

#include "./chapters/literature_review.typ"
#pagebreak()

#include "./chapters/methodology.typ"
#pagebreak()

#include "./chapters/simulation.typ"
#pagebreak()

#include "./chapters/convex_approch.typ"
#pagebreak()

// #include "./chapters/rl_approach.typ"
// #pagebreak()

// #include "./chapters/preliminary_results.typ"
// #pagebreak()

// #include "./chapters/timeline_and_plan.typ"
// #pagebreak()
//

// #include "./chapters/conclusion.typ"
// #pagebreak()

#include "./chapters/progress_update.typ"
#pagebreak()

#include "./chapters/appendices.typ"
#pagebreak()

#bibliography
