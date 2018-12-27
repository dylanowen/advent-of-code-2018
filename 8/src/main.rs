use common::*;

struct Node {
   children: Vec<Node>,
   metadata: Vec<usize>,
}

fn main() {
   run_day("8", &|contents, _is_sample| {
      let numbers: Vec<usize> = contents.trim()
         .split(' ')
         .filter_map(|raw_num| {
            match raw_num.parse::<usize>() {
               Ok(num) => Some(num),
               _ => None
            }
         })
         .collect();

      let (root, _) = parse_node(&numbers, 0);

      a(&root);
      b(&root);
   });
}

fn parse_node(data: &Vec<usize>, start: usize) -> (Node, usize) {
   let mut offset = start;
   let children_count = data[offset];
   offset += 1;
   let metadata_count = data[offset];
   offset += 1;

   let mut children: Vec<Node> = vec![];
   for _ in 0..children_count {
      let (child, end_offset) = parse_node(data, offset);

      children.push(child);
      offset = end_offset;
   }

   let mut metadata: Vec<usize> = vec![];
   for _ in 0..metadata_count {
      metadata.push(data[offset]);
      offset += 1;
   }

   (Node {
      children,
      metadata,
   }, offset)
}

fn a(root: &Node) {
   fn sum_metadata(node: &Node) -> usize {
      let children_sum = node.children.iter()
         .fold(0, |sum, child| {
            sum + sum_metadata(child)
         });


      let metadata_sum = node.metadata.iter()
         .fold(0, |total, current| {
            total + current
         });

      children_sum + metadata_sum
   }

   let metadata_sum = sum_metadata(root);

   println!("Result A: {}", metadata_sum);
}

fn b(root: &Node) {
   fn sum_data(node: &Node) -> usize {
      if node.children.is_empty() {
         // no children so just sum our metadata
         return node.metadata.iter()
            .fold(0, |total, current| {
               total + current
            });
      }
      else {
         // we have children, so use the metadata to get our children values
         return node.metadata.iter()
            .fold(0, |total, metadata| {
               let mut child_sum = 0;
               if *metadata > 0 {
                  let child_index = metadata - 1;

                  child_sum = node.children.get(child_index)
                     .map(sum_data)
                     .unwrap_or(0);
               }

               total + child_sum
            });
      }
   }

   let data_sum = sum_data(root);

   println!("Result B: {}", data_sum);
}