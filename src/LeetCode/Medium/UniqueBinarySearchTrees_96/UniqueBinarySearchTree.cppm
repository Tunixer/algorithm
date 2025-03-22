/* 两种方法，一种是模板元编程，另一种是动态生成
 */
module;  // 告知编译器这是一个module
#include <array>
#include <iostream>

export module UniqueBinarySearchTree;
export class Solution
{
   public:
    int numTrees(int n)
    {
        return compile_time_catlan(n);
    }

    /**
     * @brief 使用动态规划计算第n个卡特兰数。
     *
     * 卡特兰数是一系列自然数，在组合数学中有许多应用。
     * 在二叉搜索树的上下文中，第n个卡特兰数表示存储值1...n的结构唯一的二叉搜索树的数量。
     *
     * @param n 要计算的卡特兰数的索引。
     * @param k 起始索引。
     * @param last 第k个卡特兰属的值。
     * @return 第n个卡特兰数。
     */
    long long dynamic_catlan(int n, int k, long long last)
    {
        long long current = 0;
        for (int i = k + 1; i <= n; i++)
        {
            current = 2 * (2 * i - 1) * last / (i + 1);
            last = current;
        }
        return current;
    }

    /**
     * @brief 模板元编程计算卡特兰数数组。
     *
     * 卡特兰数是一系列自然数，在组合数学中有许多应用。
     * 此函数使用模板元编程在编译时计算前 N 个卡特兰数，并存储在数组中。
     *
     * @tparam N 要计算的卡特兰数的数量。
     * @return 包含前 N 个卡特兰数的数组。
     */
    template <std::size_t N>
    static constexpr std::array<unsigned long long, N> template_catlan()
    {
        std::array<unsigned long long, N> arr{};
        if constexpr (N > 0)
        {
            arr[0] = 1;
            if constexpr (N > 1)
            {
                for (std::size_t i = 1; i < N; ++i)
                {
                    arr[i] = 2 * (2 * static_cast<unsigned long long>(i) - 1) * arr[i - 1] /
                             (static_cast<unsigned long long>(i) + 1);
                }
            }
        }
        return arr;
    }

    int compile_time_catlan(int n)
    {
        constexpr auto arr = template_catlan<10>();
        if (n < 10)
            return static_cast<int>(arr[static_cast<std::size_t>(n)]);
        else
            return static_cast<int>(dynamic_catlan(n, 9, arr[9]));
    }

    void test()
    {
        std::cout << numTrees(19) << std::endl;
    }
};