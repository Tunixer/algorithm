/* 两种方法，一种是模板元编程，另一种是动态生成
 */
module;  // 告知编译器这是一个module
#include <iostream>
export module UniqueBinarySearchTree;
export class Solution
{
   public:
    int numTrees(int n)
    {
        return dynamic_catlan(n);
    }

    /**
     * @brief Calculate the nth Catalan number using dynamic programming.
     *
     * This function uses dynamic programming to calculate the nth Catalan
     * number. The Catalan numbers are a sequence of natural numbers that have
     * many applications in combinatorial mathematics. In the context of binary
     * search trees, the nth Catalan number represents the number of
     * structurally unique BSTs that store values 1...n.
     *
     * @param n The index of the Catalan number to calculate.
     * @return The nth Catalan number.
     */
    long long dynamic_catlan(int n)
    {
        // Base case: if n is 0, the Catalan number is 1.
        if (n == 0)
        {
            return 1;
        }
        long long last = 1, current = 1;
        for (int i = 1; i <= n; i++)
        {
            current = 2 * (2 * i - 1) * last / (i + 1);
            last = current;
        }
        return current;
    }

    void test()
    {
        std::cout << numTrees(3) << std::endl;
    }
};