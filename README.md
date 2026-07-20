# 🦀 Rust Piscine — Zone01 Oujda

Welcome to my repository containing solutions for the **Rust Piscine** at [Zone01 Oujda](https://zone01oujda.ma/). 

This intensive 4-week piscine covers foundational and advanced topics in Rust, transitioning from basic control flows to smart pointers, low-level memory layout, and custom data structures.

---

## 💡 What I Learned

Fighting the Rust borrow checker is a real rite of passage! Moving through this curriculum gave me a whole new perspective on low-level systems programming:

- **Ownership & Borrowing** — Mastering how Rust tracks data ownership and enforces mutability and reference rules strictly at compile time rather than relying on runtime guesses.
- **Lifetimes (`'a`)** — Understanding how the compiler verifies reference validity across scopes to eliminate dangling pointers.
- **Under the Hood: `libc` & Memory Management** — Rust doesn't allocate memory out of thin air. Digging deeper into Rust's heap abstractions (`Box`, `Rc`, `RefCell`) showed me how Rust interface with system C libraries (`libc`) using underlying calls like `malloc` and `free`. Rust wraps these classic C memory primitives inside strict ownership semantics, giving you C-like performance without the runtime segfaults or memory leaks.

---

## 🗺️ Curriculum Overview

### 🗓️ Week One: Foundations, Memory Safety & Collections
* **Topics:** Primitives, Ownership, Borrowing, Structs, Enums, HashMaps, Strings.
* **Quest 01:** `fibonacci2` • `scalar` • `temperature_conv` • `looping` • `speed_transformation` • `groceries` • `reverse_string` • `find_factorial` • `matrix_transposition` • `division_and_remainder` • `tuples_refs`
* **Quest 02:** `ownership` • `copy` • `borrow` • `doubtful` • `borrow_me_the_reference` • `changes` • `string_literals` • `name_initials` • `arrange_it` • `tic_tac_toe`
* **Quest 03:** `circle` • `card_deck` • `arrays` • `strings` • `edit_distance` • `to_url` • `capitalizing` • `hashing` • `string_permutation` • `bigger` • `simple_hash` • `collect`
* **Raid 01:** `drawing`

### 🗓️ Week Two: Error Handling, Structs & Enums
* **Topics:** `Option`, `Result`, Panic handling, Custom Error Types, Pattern Matching.
* **Quest 04:** `unwrap_or_expect` • `panic` • `handling` • `profanity_filter` • `question_mark` • `banner` • `cipher` • `error_types` • `boxing_todo`
* **Quest 05:** `middle_day` • `does_it_fit` • `macro_calculator` • `shopping_mall` • `expected_variable` • `mobs`
* **Raid 02:** `road-intersection`

### 🗓️ Week Three: Generics, Lifetimes & Smart Pointers
* **Topics:** Trait bounds, Explicit Lifetimes, Smart Pointers (`Box`, `Rc`, `RefCell`), Interior Mutability.
* **Quest 06:** `generics` • `traits` • `lifetimes` • `lalgebra_scalar` • `matrix` • `matrix_ops` • `matrix_mult` • `lalgebra_vector` • `blood_types` • `border_cross` • `roman_numbers` • `roman_numbers_iter` • `vectors_operations` • `events` • `delete_prefix` • `commits_stats`
* **Quest 07:** `box_it` • `borrow_box` • `box_recursion` • `how_many_references` • `ref_cell` • `drop_the_thread`
* **Project 01:** `chaikin`

### 🗓️ Week Four: Functional Rust & Advanced Iterators
* **Topics:** Closures, Higher-Order Functions, Custom Iterators, String Parsing.
* **Quest 08:** `closures` • `sales` • `adding` • `adding_twice` • `get_products` • `highest` • `iterators` • `slices_to_map` • `step_iterator` • `project_motion`
* **Quest 09:** `stars` • `ordinal` • `pangram` • `diamond_creation` • `scores` • `talking` • `searching` • `logic_number` • `rot` • `pig_latin` • `spelling` • `rgb_match`

---

## 🎨 Interactive Whiteboards (Excalidraw Diagrams)

During the piscine, I visualised complex concepts, memory ownership graphs, and problem-solving structures. You can check out the diagrams here:

| Module | Visual Notes & Flowcharts |
| :--- | :--- |
| **Quest 01** | [View Excalidraw Diagram](https://excalidraw.com/#json=xyo2PctFmRmr8J6chbf2a,WDSd5B7NVUWZmklg2-XenA) |
| **Quest 02** | [View Excalidraw Diagram](https://excalidraw.com/#json=TXToT0vg7yAMlIxin87-4,NWgTbYaDulx6vL-ZCQn6dg) |
| **Quest 03** | [View Excalidraw Diagram](https://excalidraw.com/#json=CS0OXOo7GJNPlU0FW_Xt5,WGSAPfG2kUcBlJqueLOO5Q) |
| **Checkpoint 01** | [View Diagram 1](https://excalidraw.com/#json=vHq72970ENlx06l-lDdGY,pc_3xAOcQV6yWV4vIHkITA) |
| **Raid 01** | [View Diagram](https://excalidraw.com/#json=QddOST9TPrLvD-wtH86t-,F5Zm4oGW0kPfD74ZoRd6aA) |
| **Quest 04** | [View Excalidraw Diagram](https://excalidraw.com/#json=iREHGkA7Cum5XfEGqXXoG,N95QNKmjUwG6WX5-VvAy3g) |
| **Quest 05** | [View Excalidraw Diagram](https://excalidraw.com/#json=T6Yx_8OjtyOKVacswbmMq,KdSB5YNnt6zX4NACnqZHyQ) |
| **Quest 06** | [View Excalidraw Diagram](https://excalidraw.com/#json=ik79yRwafYRCsqs4ZpNHA,Xu33-nn9yvn-p_SM8PQdKQ) |
| **Checkpoint 02** | [View Diagram 2](https://excalidraw.com/#json=V1lbsa0ah8fzvfadvkVo8,4tTZQC7fsCi7ThZV6QYwAQ) |
| **Raid 02** | [View Diagram](https://excalidraw.com/#json=aC3X7ePKVYCROkKBJ1ACh,v3qqH5xUCx_-NaGH9XoBAg) |
| **Quest 07** | [View Excalidraw Diagram](https://excalidraw.com/#json=kpj8Q-LwQH417F-qaiOyr,cRrsz_0OXBrWyF4g75nPHQ) |
| **Quest 08** | [View Excalidraw Diagram](https://excalidraw.com/#json=ZGPY0fBcNSReO1e6b6NS6,BcGqRtzH3dtHfnt5QFHe5w) |
| **Checkpoint 03** | [View Diagram 3](https://excalidraw.com/#json=id6417ZTr6xKkuK0Wn1BY,bhpjXuUVgwJon2prMU46lQ) • [Additional Notes](https://excalidraw.com/#json=hi6B-rMIBt-mUn9GOaMrs,HrX8Zd4UUaFFbqSBXO9R5Q) |

---

## ⚡ Quick Start

To run tests or build any of the individual exercise solutions:

1. **Clone the repository:**
   ```bash
   git clone [https://github.com/YOUR_USERNAME/YOUR_REPO_NAME.git](https://github.com/YOUR_USERNAME/YOUR_REPO_NAME.git)
   cd YOUR_REPO_NAME