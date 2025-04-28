#import "@preview/fletcher:0.5.7" as fletcher: diagram, node, edge

#set text(10pt)


#align(center)[
  #diagram(
    node-corner-radius: 1em,
    spacing: 2em,
    cell-size: 10mm,
    edge-stroke: 0.7pt,
    node-stroke: 0.7pt,

    // nodes
    node((2, 0), align(center)[Convex Optimization Algorithm], name: <CO>, fill: blue.lighten(75%)),

    node((2, 1), [Guidance Controller], name: <GC>, fill: green.lighten(75%)),

    node((0, 2), [Nozzle Angle PID], name: <PID1>),
    node((1, 2), [Pitch PID], name: <PID2>),
    node((2, 2), [Yaw PID], name: <PID3>),
    node((3, 2), [RCS PID], name: <PID4>),
    node((4, 2), [Engine Thrust PID], name: <PID5>),

    node((0, 3), [Nozzle Actuator], name: <ACT1>),
    node((1, 3), [Pitch Actuator], name: <ACT2>),
    node((2, 3), [Yaw Actuator], name: <ACT3>),
    node((3, 3), [RCS Actuator], name: <ACT4>),
    node((4, 3), [Engine Thrust Actuator], name: <ACT5>),

    node((2, 4), align(center)[Ship State/Sensors], name: <FS>, fill: gray.lighten(80%)),

    edge(<CO>, <GC>, "->"),
    edge(<FS>, (1.5, 4), (1.5, 1), <GC>, "-->", corner-radius: 10pt),

    edge(<GC>, <PID1>, "->"),
    edge(<GC>, <PID2>, "->"),
    edge(<GC>, <PID3>, "->"),
    edge(<GC>, <PID4>, "->"),
    edge(<GC>, <PID5>, "->"),

    edge(<PID1>, <ACT1>, "->"),
    edge(<PID2>, <ACT2>, "->"),
    edge(<PID3>, <ACT3>, "->"),
    edge(<PID4>, <ACT4>, "->"),
    edge(<PID5>, <ACT5>, "->"),

    edge(<ACT1>, (0, 3.5), (2, 3.5), <FS>, "->"),
    edge(<ACT2>, (1, 3.5), (2, 3.5), <FS>, "->"),
    edge(<ACT3>, (2, 3.5), <FS>, "->"),
    edge(<ACT4>, (3, 3.5), (2, 3.5), <FS>, "->"),
    edge(<ACT5>, (4, 3.5), (2, 3.5), <FS>, "->"),
  )
]
