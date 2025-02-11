/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
package reversenodesinkgroup
import "algorithm/src/base"

type ListNode = base.ListNode // 定义一个别名
func reverseKGroup(head *ListNode, k int) *ListNode {
	return head
}

func Test() {
	head := ListNode{Val: 1}
	reverseKGroup(&head, 0)
}