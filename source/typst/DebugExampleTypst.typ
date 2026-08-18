#import "packages/stellar.typ": *
#import "packages/definitions.typ": *
#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge

#show: stellar

// Diagram helpers corresponding to the original tikz-cd figures.
#let product-universal-diagram() = align(center, diagram(
  spacing: 18mm,
  node((-1, 0), $A$),
  node((1, 0), $B$),
  node((0, 1), $A times B$),
  node((0, 2), $X$),
  edge((0, 1), (-1, 0), $π_A$, "-|>"),
  edge((0, 1), (1, 0), $π_B$, "-|>"),
  edge((0, 2), (0, 1), $exists ! ξ$, "--|>"),
  edge((0, 2), (-1, 0), $f$, "-|>", bend: 18deg),
  edge((0, 2), (1, 0), $g$, "-|>", bend: -18deg),
))

#let product-comparison-diagram(
  upper,
  lower,
  upper-left,
  upper-right,
  lower-left,
  lower-right,
  comparison,
) = align(center, diagram(
  spacing: 18mm,
  node((-1, 0), $A$),
  node((1, 0), $B$),
  node((0, 1), upper),
  node((0, 2), lower),
  edge((0, 1), (-1, 0), upper-left, "-|>"),
  edge((0, 1), (1, 0), upper-right, "-|>"),
  edge((0, 2), (0, 1), comparison, "--|>"),
  edge((0, 2), (-1, 0), lower-left, "-|>", bend: 18deg),
  edge((0, 2), (1, 0), lower-right, "-|>", bend: -18deg),
))

#let product-isomorphism-diagram() = align(center, diagram(
  spacing: 18mm,
  node((-1, 0), $A$),
  node((1, 0), $B$),
  node((0, 1), $P$),
  node((0, 2), $Q$),
  edge((0, 1), (-1, 0), $π_A$, "-|>"),
  edge((0, 1), (1, 0), $π_B$, "-|>"),
  edge((0, 2), (-1, 0), $ρ_A$, "-|>", bend: 18deg),
  edge((0, 2), (1, 0), $ρ_B$, "-|>", bend: -18deg),
  edge((0, 2), (0, 1), $exists ! h$, "--|>", bend: 15deg),
  edge((0, 1), (0, 2), $exists ! k$, "--|>", bend: 15deg),
))

#let product-endomorphism-diagram() = align(center, diagram(
  spacing: 18mm,
  node((-1, 0), $A$),
  node((1, 0), $B$),
  node((0, 1), $P$),
  node((0, 2), $P$),
  edge((0, 1), (-1, 0), $π_A$, "-|>"),
  edge((0, 1), (1, 0), $π_B$, "-|>"),
  edge((0, 2), (-1, 0), $π_A$, "-|>", bend: 18deg),
  edge((0, 2), (1, 0), $π_B$, "-|>", bend: -18deg),
  edge((0, 2), (0, 1), $exists ! φ$, "--|>"),
))

#let coproduct-universal-diagram() = align(center, diagram(
  spacing: 18mm,
  node((-1, 0), $A$),
  node((1, 0), $B$),
  node((0, 1), $A ⊔ B$),
  node((0, 2), $X$),
  edge((-1, 0), (0, 1), $ι_A$, "-|>"),
  edge((1, 0), (0, 1), $ι_B$, "-|>"),
  edge((0, 1), (0, 2), $exists ! ξ$, "--|>"),
  edge((-1, 0), (0, 2), $f$, "-|>", bend: -18deg),
  edge((1, 0), (0, 2), $g$, "-|>", bend: 18deg),
))

#id("debug-example-typst")
#genpage()

#stellar-section("Product")

#snippetdefinition("category-product-definition2", [Product])[
  Let $cal(C)$ be a #category(), and let $A, B insym catob (cal(C))$.
  A _product_ of $A$ and $B$ in $cal(C)$ is an object
  $A times B insym catob (cal(C))$ together with two morphisms

  $ π_A : A times B fromto A, quad π_B : A times B fromto B $

  #grid(
    columns: (65%, 35%),
    column-gutter: 0pt,
    align: (top + left, top + center),
    [
      called _projections_, such that the following universal property holds:
      #linebreak()
      for all $X insym catob (cal(C))$ with morphisms
      $f : X fromto A$ and $g : X fromto B$, there exists a unique morphism
      $ξ : X fromto A times B$ such that

      $ π_A compose ξ = f quad land quad π_B compose ξ = g $
    ],
    [#product-universal-diagram()],
  )
]

#snippettheorem("category-product-in-set-theorem2", [Product in $bold("Set")$])[
  Let $A, B insym catob (bold("Set"))$. Then, the categorical product of
  $A$ and $B$ corresponds to the cartesian product $A cartesianprod B$.
]

#snippetproof(
  "category-product-in-set-theorem-proof2",
  "category-product-in-set-theorem2",
  [Product in $bold("Set")$],
)[
  We will prove that
  $A times B = \{(a, b) suchthat a insym A land b insym B\}$.
  Define $π_A((a, b)) = a$ and $π_B((a, b)) = b$.
  Given any #set-term() $X$ and #function(text: [functions])
  $f : X fromto A$ and $g : X fromto B$, we need to find a unique
  #function() $ξ : X fromto A cartesianprod B$ such that

  $ π_A(ξ(x)) = f(x) land π_B(ξ(x)) = g(x) $

  Let $ξ(x) = (f(x), g(x))$. Then, we have

  $ π_A(ξ(x)) &= π_A((f(x), g(x))) = f(x) \
    π_B(ξ(x)) &= π_B((f(x), g(x))) = g(x) $

  Let $ξ' : X fromto A cartesianprod B$ satisfy those equations.
  But then we must have $ξ'(x) = (f(x), g(x))$, and thus $ξ = ξ'$.
]

#plainhtml("We will prove that every universal construction is unique up to isomorphism. We can prove it in this specific case to understand the argument.")

#snippetproposition("categorical-product-unique-isomorphism2", [])[
  The categorical product is unique up to isomorphism.
]

#snippetproof(
  "categorical-product-unique-isomorphism-proof2",
  "categorical-product-unique-isomorphism2",
  [],
)[
  Let $A, B insym catob (bold("Set"))$. Consider two categorical products
  of $A$ and $B$, $P$ and $Q$, with projections $π_A, π_B$ and $ρ_A, ρ_B$,
  and unique morphisms $h : X fromto P$, $k : X fromto Q$ respectively.
  Since the universal property requires commutativity for all
  $X insym catob (bold("Set"))$, we can choose the particular case $X = Q$
  for the definition of $P$ and $X = P$ for the definition of $Q$.
  This gives us the following diagrams:

  #grid(
    columns: (1fr, 1fr),
    column-gutter: 0pt,
    align: center,
    [#product-comparison-diagram(
      $P$, $Q$, $π_A$, $π_B$, $ρ_A$, $ρ_B$, $exists ! h$,
    )],
    [#product-comparison-diagram(
      $Q$, $P$, $ρ_A$, $ρ_B$, $π_A$, $π_B$, $exists ! k$,
    )],
  )

  We will now prove that $h$ and $k$ are inverses of each other:

  #product-isomorphism-diagram()

  Let $φ = h compose k$. Then, since

  $ π_i compose (h compose k)
    = (π_i compose h) compose k
    = ρ_i compose k
    = π_i, quad i insym \{1, 2\} $

  the following diagram is commutative:

  #product-endomorphism-diagram()

  However, if we let $φ = "id"_P$, the resulting diagram is also commutative.
  Since $φ$ is unique, then we must have $φ = "id"_P$ and thus
  $h compose k = "id"_P$. By swapping the roles of $P$ and $Q$, we also get
  $k compose h = "id"_Q$.
]

#stellar-section("Coproduct")

#snippetdefinition("category-coproduct-definition2", [Coproduct])[
  Let $cal(C)$ be a #category(), and let $A, B insym catob (cal(C))$.
  A _coproduct_ of $A$ and $B$ in $cal(C)$ is an object
  $A ⊔ B insym catob (cal(C))$ together with two morphisms

  $ ι_A : A fromto A ⊔ B, quad ι_B : B fromto A ⊔ B $

  #grid(
    columns: (65%, 35%),
    column-gutter: 0pt,
    align: (top + left, top + center),
    [
      called _injections_, such that the following universal property holds:
      #linebreak()
      for all $X insym catob (cal(C))$ with morphisms
      $f : A fromto X$ and $g : B fromto X$, there exists a unique morphism
      $ξ : A ⊔ B fromto X$ such that

      $ ξ compose ι_A = f quad land quad ξ compose ι_B = g $
    ],
    [#coproduct-universal-diagram()],
  )
]

#snippettheorem("category-coproduct-in-set-theorem2", [Coproduct in $bold("Set")$])[
  Let $A, B insym catob (bold("Set"))$. Then, the categorical coproduct of
  $A$ and $B$ corresponds to the disjoint union $A disjointunion B$.
]

#snippetproof(
  "category-coproduct-in-set-theorem-proof2",
  "category-coproduct-in-set-theorem2",
  [Coproduct in $bold("Set")$],
)[
  We will prove that
  $A ⊔ B = (A cartesianprod \{0\}) union (B cartesianprod \{1\})$.
  Define $ι_A(a) = (a, 0)$ and $ι_B(b) = (b, 1)$.
  Given any #set-term() $X$ and #function(text: [functions])
  $f : A fromto X$ and $g : B fromto X$, we need to find a unique
  #function() $ξ : A disjointunion B fromto X$ such that

  $ ξ(ι_A(x)) = f(x) land ξ(ι_B(x)) = g(x) $

  Let

  $ ξ((x, i)) = cases(
      f(x) & i = 0,
      g(x) & i = 1,
    ), quad i insym \{0, 1\} $

  Then, we have

  $ ξ(ι_A) &= ξ((x, 0)) = f(x) \
    ξ(ι_B) &= ξ((x, 1)) = g(x) $

  Let $ξ' : A disjointunion B fromto X$ satisfy those equations.
  But then we must have

  $ ξ'((x, i)) = cases(
      f(x) & i = 0,
      g(x) & i = 1,
    ), quad i insym \{0, 1\} $

  and thus $ξ = ξ'$.
]
