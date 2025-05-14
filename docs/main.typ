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
    key: "qp",
    short: "QP",
    long: "Quadratic Programming",
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
    key: "3dof",
    short: "3DOF",
    long: "Three Degrees of Freedom",
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
    key: "lc",
    short: "LCvx",
    long: "Lossless Convexification",
  ),
  (
    key: "ipm",
    short: "IPM",
    long: "Interior Point Method",
  ),
  (
    key: "ppo",
    short: "PPO",
    long: "Proximal Policy Optimisation",
  ),
  (
    key: "sac",
    short: "SAC",
    long: "Soft Actor-Critic",
  ),
  (
    key: "drl",
    short: "Deep-RL",
    long: "Deep Reinforcement Learning",
  ),
  (
    key: "zemzev",
    short: "ZEM/ZEV",
    long: "Zero-Effort-Miss/Zero-Effort-Velocity",
  ),
  (
    key: "lvs",
    short: "LVS",
    long: "Lander Vision System",
  ),
  (
    key: "trn",
    short: "TRN",
    long: "Terrain Relative Navigation",
  ),
  (
    key: "lstm",
    short: "LSTM",
    long: "Long Short-Term Memory",
  ),
  (
    key: "mpc",
    short: "MPC",
    long: "Model Predictive Control",
  ),
  (
    key: "rtls",
    short: "RTLS",
    long: "Return to Launch Site",
  ),
  (
    key: "ddpg",
    short: "DDPG",
    long: "Deep Deterministic Policy Gradient",
  ),
  (
    key: "td3",
    short: "TD3",
    long: "Twin Delayed DDPG",
  ),
  (
    key: "sd3",
    short: "SD3",
    long: "Softmax Double DDPG",
  ),
)


#register-glossary(abbreviations)

#show: codly-init
#codly(zebra-fill: none, languages: codly-languages)


#set text(font: "Times New Roman", size: 12pt, fill: rgb(51, 51, 51))
#show raw: set text(font: "Monaspace Argon", size: 10pt, fill: rgb(80, 80, 80))
#set par(leading: 0.7em)

// Define bibliography
#let bibliography = bibliography("references.bib", style: "ieee", title: "References")

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

// Abstract
#include "./chapters/abstract.typ"
#pagebreak()

// Table of contents
#outline(
  title: [Contents],
  indent: 1em,
  depth: 1,
)
#pagebreak()

// List of figures
#outline(
  title: [List of Figures],
  target: figure.where(kind: image),
)

// List of tables
// #outline(
//   title: [List of Tables],
//   target: figure.where(kind: table),
// )
#pagebreak()

// Abbreviations
#abbreviations-page(abbreviations)
#pagebreak()

#include "./chapters/introduction.typ"
#pagebreak()

#include "./chapters/literature_review.typ"
#pagebreak()

#include "./chapters/problem_analysis.typ"
#pagebreak()

#include "./chapters/technology_stack.typ"
#pagebreak()

#include "./chapters/background.typ"
#pagebreak()

#include "./chapters/implementation/index.typ"
#pagebreak()

#include "./chapters/methodology.typ"
#pagebreak()

#include "./chapters/results.typ"
#pagebreak()

#include "./chapters/conclusion.typ"

#bibliography

#include "./chapters/appendices.typ"
#pagebreak()

