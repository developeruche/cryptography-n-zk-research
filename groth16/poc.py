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
    print(out)
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
    
    print("Ua, Va, Wa")
    print(Ua, Va, Wa)
    print("==========")

    t = galois.Poly([1, curve_order-1], field=GF) * galois.Poly([1, curve_order-2], field=GF) * galois.Poly([1, curve_order-3], field=GF)
    h = (Ua * Va - Wa) // t

    # The equation is then Uw Vw = Ww + h t
    assert Ua * Va == Wa + h * t, "Ua * Va != Wa + h(x)t(x)"

    return Ua, Va, Wa, h, t, U, V, W, a, private_input_index
    
    
get_qap(2, 2)