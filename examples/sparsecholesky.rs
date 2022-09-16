/*
   Copyright: $$copyright

   File:      $${file}

   Purpose:   Demonstrate the sparse Cholesky factorization.

 */
package com.mosek.example;
import mosek.*;

public class sparsecholesky {
  public static void printsparse(int      n,
                                 int[]    perm,
                                 double[] diag,
                                 int[]    lnzc,
                                 long[]   lptrc,
                                 long     lensubnval,
                                 int[]    lsubc,
                                 double[] lvalc) {
    int i, j;
    System.out.print("P = [ ");
    for (i = 0; i < n; i++) System.out.print(perm[i] + " ");
    System.out.println("]");
    System.out.print("diag(D) = [ ");
    for (i = 0; i < n; i++) System.out.print(diag[i] + " ");
    System.out.println("]");
    double[] l = new double[n * n];
    for (j = 0; j < n; j++)
      for (i = (int)lptrc[j]; i < lptrc[j] + lnzc[j]; i++)
        l[lsubc[i] * n + j] = lvalc[i];
    System.out.println("L=");
    for (i = 0; i < n; i++) {
      for (j = 0; j < n; j++) System.out.print(l[i * n + j] + " ");
      System.out.println("");
    }
  }


  public static void main(String[] args) {
    /* Create the mosek environment. */
    try (Env env = new Env()) {
      {
        //Example from the manual
        /*TAG:begin-example1*/
        //Observe that anzc, aptrc, asubc and avalc only specify the lower triangular part.
        int n                 = 4;
        int[] anzc            = {4, 1, 1, 1};
        int[] asubc           = {0, 1, 2, 3, 1, 2, 3};
        long[] aptrc          = {0, 4, 5, 6};
        double[] avalc        = {4.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0};
        double[] b            = {13.0, 3.0, 4.0, 5.0};
        /*TAG:end-example1*/

        int[][] perm      = new int[1][];
        int[][] lnzc      = new int[1][];
        int[][] lsubc     = new int[1][];
        long[] lensubnval = new long[1];
        double[][] diag   = new double[1][];
        long[][] lptrc    = new long[1][];
        double[][] lvalc  = new double[1][];

        /*TAG:begin-factorization*/
        env.computesparsecholesky(0,        //Mosek chooses number of threads
                                  1,        //Apply reordering heuristic
                                  1.0e-14,  //Singularity tolerance
                                  anzc, aptrc, asubc, avalc,
                                  perm, diag,
                                  lnzc, lptrc, lensubnval, lsubc, lvalc);

        printsparse(n, perm[0], diag[0], lnzc[0], lptrc[0], lensubnval[0], lsubc[0], lvalc[0]);

        /* Permuted b is stored as x. */
        double[] x = new double[n];
        for (int i = 0; i < n; i++) x[i] = b[perm[0][i]];

        /*Compute  inv(L)*x.*/
        env.sparsetriangularsolvedense(mosek.transpose.no, lnzc[0], lptrc[0], lsubc[0], lvalc[0], x);
        /*Compute  inv(L^T)*x.*/
        env.sparsetriangularsolvedense(mosek.transpose.yes, lnzc[0], lptrc[0], lsubc[0], lvalc[0], x);

        System.out.print("\nSolution A x = b, x = [ ");
        for (int i = 0; i < n; i++)
          for (int j = 0; j < n; j++) if (perm[0][j] == i) System.out.print(x[j] + " ");
        System.out.println("]\n");
        /*TAG:end-factorization*/
      }

      {
        /*TAG:begin-example2 */
        int n                 = 3;
        int[] anzc            = {3, 2, 1};
        int[] asubc           = {0, 1, 2, 1, 2, 2};
        long[] aptrc          = {0, 3, 5, };
        double[] avalc        = {1.0, 1.0, 1.0, 1.0, 1.0, 1.0};
        /*TAG:end-example2 */

        int[][] perm      = new int[1][];
        int[][] lnzc      = new int[1][];
        int[][] lsubc     = new int[1][];
        long[] lensubnval = new long[1];
        double[][] diag   = new double[1][];
        long[][] lptrc    = new long[1][];
        double[][] lvalc  = new double[1][];

        env.computesparsecholesky(0,        //Mosek chooses number of threads
                                  1,        //Apply reordering heuristic
                                  1.0e-14,  //Singularity tolerance
                                  anzc, aptrc, asubc, avalc,
                                  perm, diag,
                                  lnzc, lptrc, lensubnval, lsubc, lvalc);

        printsparse(n, perm[0], diag[0], lnzc[0], lptrc[0], lensubnval[0], lsubc[0], lvalc[0]);
      }
    } catch (mosek.Exception e) {
      System.out.println("An error was encountered");
      System.out.println(e.getMessage());
      throw e;
    }
  }
}
