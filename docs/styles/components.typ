// filepath: /Users/fionnbarrett/Documents/programming/land-rs/docs/styles/components.typ
#let info-box(body) = {
  block(
    fill: rgb(240, 249, 255),
    inset: 12pt,
    radius: 4pt,
    stroke: rgb(212, 235, 255),
    width: 100%,
  )[
    #text(weight: "medium", fill: rgb(40, 120, 200))[Info]
    #v(0.5em)
    #body
  ]
}

#let warning-box(body) = {
  block(
    fill: rgb(255, 248, 240),
    inset: 12pt,
    radius: 4pt,
    stroke: rgb(255, 235, 212),
    width: 100%,
  )[
    #text(weight: "medium", fill: rgb(200, 120, 40))[Warning]
    #v(0.5em)
    #body
  ]
}

#let note-box(body) = {
  block(
    fill: rgb(240, 240, 240),
    inset: 12pt,
    radius: 4pt,
    stroke: rgb(220, 220, 220),
    width: 100%,
  )[
    #text(weight: "medium", fill: rgb(80, 80, 80))[Note]
    #v(0.5em)
    #body
  ]
}


// Figure styling for images
#let clean-figure(caption, img) = figure(
  img,
  caption: caption,
)

// Show figure with custom styling
#show figure: fig => {
  set align(center)
  fig.body
  v(0.5em)
  text(size: 10pt, fill: rgb(100, 100, 100), weight: "regular", fig.caption)
  v(1em)
}

// Clean definition list
#let clean-def-list(..items) = {
  for item in items.pos() {
    let (term, definition) = item
    [
      #text(weight: "medium", term)
      #v(4pt)
      #block(inset: (left: 1em))[#definition]
    ]
    v(0.8em)
  }
}
