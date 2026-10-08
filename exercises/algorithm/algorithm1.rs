/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
	pub fn merge(mut list_a:LinkedList<T>,mut list_b:LinkedList<T>) -> Self where T: Ord,
	{
            // 1. 保存合并后的总长度
        let total = list_a.length
            .checked_add(list_b.length)
            .expect("list length overflow");

        // 2. 取出两个链表的头节点和尾节点信息
        let mut a = list_a.start.take();
        let mut b = list_b.start.take();

        let end_a = list_a.end.take();
        let end_b = list_b.end.take();

        // 原链表不再管理这些节点
        list_a.length = 0;
        list_b.length = 0;

        let mut result = Self::new();

        // 3. 两条链表都还有节点时，比较头节点
        while let (Some(pa), Some(pb)) = (a, b) {
            // SAFETY: pa、pb 指向有效、已初始化的独立节点。
            let take_a = unsafe {
                pa.as_ref().val <= pb.as_ref().val
            };

            // 4. 选择较小节点，并推进对应的游标
            let mut chosen = if take_a {
                a = unsafe { pa.as_ref().next };
                pa
            } else {
                b = unsafe { pb.as_ref().next };
                pb
            };

            // 5. 断开选中节点的旧 next
            // SAFETY: chosen 是有效且由合并过程独占管理的节点。
            unsafe {
                chosen.as_mut().next = None;
            }

            // 6. 将 chosen 接到结果链表尾部
            match result.end {
                None => result.start = Some(chosen),
                Some(mut tail) => unsafe {
                    // SAFETY: tail 是结果链表的有效尾节点，
                    // 此时没有其他活跃引用访问该节点。
                    tail.as_mut().next = Some(chosen);
                },
            }

            result.end = Some(chosen);
        }

        // 7. 至少一个输入链表已经耗尽，
        //    找到另一条链表剩余的节点。
        let (rest, last) = if a.is_some() {
            (a, end_a)
        } else {
            (b, end_b)
        };

        // 8. 直接接上剩余链表
        match result.end {
            None => result.start = rest,
            Some(mut tail) => unsafe {
                // SAFETY: tail 是有效尾节点，rest 是未处理的独立链。
                tail.as_mut().next = rest;
            },
        }

        if rest.is_some() {
            result.end = last;
        }

        result.length = total;
        result
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}