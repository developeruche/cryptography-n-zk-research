# Polynomials
--------------------

Polynomials, those elegant expressions of coefficients and variables, hold within them the power to transcend mere mathematics and touch the realms of art, security, and computation. They are the silent composers of symphonies that secure our digital secrets, the architects of algorithms, and the timeless dancers in the space of numbers.

In cryptography, polynomials become the guardians of our privacy. Imagine them as the secretive sentinels of the digital age, weaving intricate webs through finite fields and elliptic curves. They hide messages within their degrees and coefficients, creating ciphers that are as impenetrable as a knight’s armor. The beauty of polynomial-based cryptographic methods, like those in elliptic curve cryptography, lies in their blend of simplicity and complexity. A single polynomial equation, when viewed through the lens of number theory, can secure communications against the most relentless of adversaries.

Mathematically, polynomials are the storytellers of calculus and algebra, narrating the tales of curves and surfaces. They are the chameleons of mathematics, capable of transforming into various forms—factored, expanded, or simplified. Each transformation reveals a different facet of their character, whether it’s the roots that solve equations or the coefficients that describe geometric shapes. The Fundamental Theorem of Algebra whispers the promise that every non-zero polynomial has at least one complex root, a promise that ensures completeness in the mathematical universe.

To a programmer, polynomials are the unsung heroes of code. They are the engines driving graphics rendering, error detection, and machine learning algorithms. Picture a polynomial as a loyal steed, carrying a programmer through the treacherous terrain of computational complexity. In computer graphics, Bézier curves, those graceful polynomial curves, render the smooth lines and shapes that we see on our screens. Error-correcting codes like Reed-Solomon, built upon the foundation of polynomials, ensure the integrity of our data as it travels across the globe.

Polynomials, thus, are not just mathematical constructs. They are the bridge between the abstract and the tangible, the simple and the profound. They dance in the cryptographic shadows, tell stories in the language of algebra, and work tirelessly in the digital realms crafted by programmers. In their coefficients and variables, they hold the essence of order and chaos, a delicate balance that makes the world of mathematics so infinitely fascinating.


## Polynomial

This is the implementation of a polynomial in Rust. The Polynomial struct allows you to create a polynomial, evaluate it at a specific point, and add or multiply two polynomials together.

The variations of polynomials built in here are;
- Univariate Polynomial
- Multivariate Polynomial
- Multilinear Polynomial

... the last two could give a man 2^N complexity nightmare :).