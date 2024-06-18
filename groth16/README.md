## The Groth16 Protocol 
---------------------------


prerequisite for this study are;
- Bilinear pairings and elliptic curves
- Quadratic arithmetic programs (QAPs)

Bilinear pairings and elliptic curves are fundamental to the Groth16 proving system and many other advanced cryptographic protocols. Elliptic curves provide a secure and efficient foundation, while bilinear pairings enable the construction of succinct, non-interactive proofs that are both powerful and practical for real-world applications.


#### Elliptic Curves

**Elliptic Curves Overview:** An elliptic curve is a set of points that satisfy a specific mathematical equation. Over a finite field $\mathbb{F}_q​$, an elliptic curve $E$ is typically defined by an equation of the form: $y^2 = x^3 + ax + b$ where $a$ and $b$ are coefficients in $\mathbb{F}_q$, and the curve is non-singular (i.e., it has no cusps or self-intersections), which requires the discriminant $4a^3 + 27b^2 \neq 0$.

### Bilinear Pairings in Groth16

In the context of the Groth16 proving system, bilinear pairings are used to construct efficient proofs that are both small in size and quick to verify. The system relies on the properties of bilinear pairings to ensure that the proof verification can be performed using simple algebraic operations, making the process efficient even for complex statements.

**Example Use in Groth16:**

- **Setup Phase:** During the setup phase, a trusted party generates public parameters that include elements in $G_1$​, $G_2$​, and $G_T$.
- **Proving Phase:** The prover uses these parameters to create a proof that certain computations were performed correctly. This involves operations in $G1$​ and $G2$.
- **Verification Phase:** The verifier checks the proof by performing pairing operations $e$ and ensures that certain equations hold in $G_T$​. The bilinear nature of the pairing allows these checks to be performed efficiently.

### The Protocol 
The `Groth16` protocol in most cases, starts with a Circuit. A `Circuit`  in ZKP is a structured way to represent a computation. It is analogous to a digital logic circuit used in computer engineering. 

**Structure:**

- A circuit is composed of **inputs**, **gates**, and **outputs**.
- **Inputs**: These are the values that the prover knows and wants to keep private (often called witnesses) along with the public inputs.
- **Gates**: These perform basic operations (such as addition, multiplication, logical AND, OR) on the inputs.
- **Outputs**: These are the results of the computations performed by the gates.

**Types of Circuits:**

- **Arithmetic Circuits**: These circuits use arithmetic gates (addition, multiplication) over a finite field. They are often used in zk-SNARKs because they can efficiently represent the types of computations involved in many cryptographic applications.
- **Boolean Circuits**: These circuits use logical gates (AND, OR, NOT) and are more common in classical computer science but can be transformed into arithmetic circuits for use in zk-SNARKs.

The next stage of the protocol is to convert this `Circuit` to a format called `R1CS` 

A rank-1 constraint system (R1CS) with n variables and m constraints and p public inputs has a witness vector $w∈F^n$. By convention, the first p elements of w are the public input and the first public input $w_0​$ is fixed to 1. The m constraints in R1CS are a product equation between three inner products:
$$
(w⋅ai​)⋅(w⋅bi​)=w⋅ci​
$$
where vectors $(a_i​,b_i​,c_i​)∈F^3⋅^n$ specify the constraint. The constraint vectors are usually very sparse, typically only nonzero for a single or few values. These constraint vectors can be aggregated in $n×m$ matrices so that the whole constraint system can be concisely written using an element-wise product.

$$
(w⋅A)∘(w⋅B)=w⋅C
$$

After obtaining this R1CS representation, it needs to be transformed into its QAP representation to go further in this proof system.
