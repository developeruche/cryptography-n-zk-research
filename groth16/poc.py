'''
Continuing work from hw7
'''

import numpy as np
from py_ecc.bls12_381 import G1, G2, add, multiply, pairing, neg, final_exponentiate, curve_order, eq, Z1, Z2, FQ, FQ2, FQ12
from ape import accounts, project
import galois
import random

# curve_order = 79
GF = galois.GF(curve_order)

def inner_product(powers_of_tau, coeffs, z):
    sum = z
    for i in range(len(coeffs)):
        pdt = multiply(powers_of_tau[i], int(coeffs[i]))
        sum = add(sum, pdt)
    return sum

def get_qap(x, y):
    def remove_negatives(row):
        return [curve_order+el if el < 0 else el for el in row] 

    # Define the matrices
    A = GF(np.apply_along_axis(remove_negatives, 1, np.array([[0,0,3,0,0,0],
                                                        [0,0,0,0,1,0],
                                                        [0,0,1,0,0,0]])))

    B = GF(np.apply_along_axis(remove_negatives, 1, np.array([[0,0,1,0,0,0],
                                                        [0,0,0,1,0,0],
                                                        [0,0,0,5,0,0]])))
    
    # np.apply_along_axis on C resulted in OverflowError: Python int too large to convert to C long
    C_raw = np.array([[0,0,0,0,1,0],
                  [0,0,0,0,0,1],
                  [-3,1,1,2,0,-1]])
    C = GF([remove_negatives(row) for row in C_raw])

    # Compute the witness
    v1 = GF(3)*x*x
    v2 = v1 * y
    out = GF(3)*x*x*y + GF(5)*x*y + GF(curve_order-1)*x + GF(curve_order-2)*y + GF(3) # out = 3x^2y + 5xy - x - 2y + 3
    w = GF(np.array([1, out, x, y, v1, v2]))
    private_input_index = 2

    # Sanity check
    assert np.all(np.equal(A.dot(w) * B.dot(w), C.dot(w))), "Aw * Bw != Cw"

    # Convert each matrix into polynomial matrices U V W using Lagrange on xs = [1,2,3] and each column of the matrices
    def interpolate_col(col):
        xs = GF(np.array([1,2,3]))
        return galois.lagrange_poly(xs, col)

    U = np.apply_along_axis(interpolate_col, 0, A)
    V = np.apply_along_axis(interpolate_col, 0, B)
    W = np.apply_along_axis(interpolate_col, 0, C)

    # Rename w as a to follow the notation on the book
    a = w

    # Compute Uw, Vw and Ww 
    Ua = U.dot(a)
    Va = V.dot(a)
    Wa = W.dot(a)

    t = galois.Poly([1, curve_order-1], field=GF) * galois.Poly([1, curve_order-2], field=GF) * galois.Poly([1, curve_order-3], field=GF)
    h = (Ua * Va - Wa) // t

    # The equation is then Uw Vw = Ww + h t
    assert Ua * Va == Wa + h * t, "Ua * Va != Wa + h(x)t(x)"

    return Ua, Va, Wa, h, t, U, V, W, a, private_input_index

# The trusted setup here is just a mock, with the values hidden from everyone
def trusted_setup(U, V, W, t, degrees, priv_idx):
    tau = GF(4)
    alpha = GF(2)
    beta = GF(3)
    gamma = GF(5)
    delta = GF(6)
    
    print("tau", tau)
    print("degree", degrees)
    
    
    powers_of_tau_1 = [multiply(G1,int(tau**i)) for i in range(degrees + 1)]
    alpha1 = multiply(G1, int(alpha))
    powers_of_tau_2 = [multiply(G2,int(tau**i)) for i in range(degrees + 1)]
    beta2 = multiply(G2, int(beta))

    # We evaluate the polynomial t at tau and multiply it with various powers of tau
    # to get a vector of (encrypted) various powers of tau that the prover can conveniently 
    # replace the powers of x with (i.e. multiply the coefficients of h with the powers of tau)
    powers_of_tau_HT = [multiply(G1, int(tau**i * t(tau) / delta)) for i in range(t.degree)]

    # Finally, we want to pre-compute W(tau) + beta*U(tau) + alpha*V(tau)).
    # This computation is different as we are given row matrixes of galois.Poly
    # e.g. U = [0, 0, 2x^2 + 70x + 10, 0, 78x^2 + 4x + 76, 0] so
    # so we need to evaluate each polynomial in the matrix at tau and scale the
    # them by alpha or beta resulting e.g. beta * U(tau) = [0, 0, 3*82, 0, 3*158, 0]
    # Then we encrypt it with G1
    W_tau = [poly(tau) for poly in W]
    U_tau = [beta * poly(tau) for poly in U]
    V_tau = [alpha * poly(tau) for poly in V]
    C_tau = [w + u + v for w, u, v in zip(W_tau, U_tau, V_tau)]

    powers_of_tau_C_public = [multiply(G1,int(c/gamma)) for c in C_tau[:priv_idx]]
    powers_of_tau_C_private = [multiply(G1,int(c/delta)) for c in C_tau[priv_idx:]]

    # Shifts for public & private witness
    gamma2 = multiply(G2, int(gamma))
    delta2 = multiply(G2, int(delta))

    # Print out for the verifier
    # print(alpha1)
    # print(beta2)
    # print(powers_of_tau_C_public)
    # print(gamma2)
    # print(delta2)

    # Shifts for r and s
    beta1 = multiply(G1, int(beta))
    delta1 = multiply(G1, int(delta))

    return powers_of_tau_1, alpha1, powers_of_tau_2, beta2, powers_of_tau_C_public, powers_of_tau_C_private, powers_of_tau_HT, gamma2, delta2, beta1, delta1

# part 3
def test_verify(accounts):
    x, y, r, s = GF.Random(4)

    # TODO: get_qap should just return private_inputs and public inputs so it's less error prone
    Ua, Va, Wa, h, t, U, V, W, a, priv_input_idx = get_qap(x,y) 

    # Remember only a is secret and cannot be passed around
    powers_of_tau_1, alpha1, powers_of_tau_2, beta2, powers_of_tau_C_public, powers_of_tau_C_private, powers_of_tau_HT, gamma2, delta2, beta1, delta1 = trusted_setup(U, V, W, t, Ua.degree, priv_input_idx)

    r1 = multiply(delta1, int(r))
    A1 = add(add(inner_product(powers_of_tau_1, Ua.coeffs[::-1], Z1), alpha1), r1)
    s2 = multiply(delta2, int(s))
    B2 = add(add(inner_product(powers_of_tau_2, Va.coeffs[::-1], Z2), beta2), s2)
    s1 = multiply(delta1, int(s))
    B1 = add(add(inner_product(powers_of_tau_1, Va.coeffs[::-1], Z1), beta1), s1)

    # Compute X for sanity checks
    X1 = inner_product(powers_of_tau_C_public, a[:priv_input_idx], Z1) 

    C_prime_1 = inner_product(powers_of_tau_C_private, a[priv_input_idx:], Z1) 
    HT1 = inner_product(powers_of_tau_HT, h.coeffs[::-1], Z1)
    sA1 = multiply(A1, int(s))
    rB1 = multiply(B1, int(r))
    rsdelta1 = multiply(delta1, int(r*s))
    C1 = add(add(add(add(C_prime_1, HT1), sA1), rB1), neg(rsdelta1))

    # Sanity checks 
    pair1 = pairing(B2, neg(A1))
    pair2 = pairing(beta2, alpha1)
    pair3 = pairing(gamma2, X1)
    pair4 = pairing(delta2, C1)
    assert final_exponentiate(pair1 * pair2 * pair3 * pair4) == FQ12.one(), "A1B2 != alpha beta + X1 gamma2 +C1 delta2"

    A1_str = [repr(el) for el in A1]
    B2_str = [[repr(el.coeffs[0]), repr(el.coeffs[1])] for el in B2]
    C1_str = [repr(el) for el in C1]
    public_inputs = [repr(int(el)) for el in a[:priv_input_idx]]

    account = accounts[0]
    contract = account.deploy(project.Groth16Verifier)
    result = contract.verify(A1_str, B2_str, C1_str, public_inputs)
    assert result
    
def test_verify_no_accounts():
    x = GF(2)
    y = GF(2)
    r = GF(3)
    s = GF(5)

    # TODO: get_qap should just return private_inputs and public inputs so it's less error prone
    Ua, Va, Wa, h, t, U, V, W, a, priv_input_idx = get_qap(x,y) 

    # Remember only a is secret and cannot be passed around
    powers_of_tau_1, alpha1, powers_of_tau_2, beta2, powers_of_tau_C_public, powers_of_tau_C_private, powers_of_tau_HT, gamma2, delta2, beta1, delta1 = trusted_setup(U, V, W, t, Ua.degree, priv_input_idx)
    
    print("Printing the trusted setup")
    print(powers_of_tau_1)
    # print(alpha1)
    # print(powers_of_tau_2)
    # print(beta2)
    # print(powers_of_tau_C_public)
    # print(powers_of_tau_C_private)
    # print(powers_of_tau_HT)
    # print(gamma2)
    # print(delta2)
    # print(beta1)
    # print(delta1)
    print("End of the trusted setup")
    
    
    
    # r1 = multiply(delta1, int(r))
    # A1 = add(add(inner_product(powers_of_tau_1, Ua.coeffs[::-1], Z1), alpha1), r1)
    # s2 = multiply(delta2, int(s))
    # B2 = add(add(inner_product(powers_of_tau_2, Va.coeffs[::-1], Z2), beta2), s2)
    # s1 = multiply(delta1, int(s))
    # B1 = add(add(inner_product(powers_of_tau_1, Va.coeffs[::-1], Z1), beta1), s1)

    # # Compute X for sanity checks
    # X1 = inner_product(powers_of_tau_C_public, a[:priv_input_idx], Z1) 

    # C_prime_1 = inner_product(powers_of_tau_C_private, a[priv_input_idx:], Z1) 
    # HT1 = inner_product(powers_of_tau_HT, h.coeffs[::-1], Z1)
    # sA1 = multiply(A1, int(s))
    # rB1 = multiply(B1, int(r))
    # rsdelta1 = multiply(delta1, int(r*s))
    # C1 = add(add(add(add(C_prime_1, HT1), sA1), rB1), neg(rsdelta1))

    # # Sanity checks 
    # pair1 = pairing(B2, neg(A1))
    # pair2 = pairing(beta2, alpha1)
    # pair3 = pairing(gamma2, X1)
    # pair4 = pairing(delta2, C1)
    # assert final_exponentiate(pair1 * pair2 * pair3 * pair4) == FQ12.one(), "A1B2 != alpha beta + X1 gamma2 +C1 delta2"

    # A1_str = [repr(el) for el in A1]
    # B2_str = [[repr(el.coeffs[0]), repr(el.coeffs[1])] for el in B2]
    # C1_str = [repr(el) for el in C1]
    # public_inputs = [repr(int(el)) for el in a[:priv_input_idx]]

    # print("Done with the verification test!")
    
    
test_verify_no_accounts()