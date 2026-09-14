# Data Structures and Algorithms
## purpose of this 
- I am not gonna do countless leet code nonsense 
- I am gonna learn things organically 

# Time complexity 
**What is Time Complexity?**

***Time complexity*** is the time taken by an *algorithm* to complete executing what it set out to do  
for example calculating the time it would take to put 100 apples in a box one by one is called time complexity  
the first programme i have written to under this is a simple insertion programme, inserting 100, 1000, 10000 numbers  
inside an array  


**Pgm 1:**  
*for size in [100usize, 1_000, 10_000, 100_000] {  
 let mut array = Vec::new();  
 let timer = Instant::now();  
for n in 0..size{  
 array.push(n);  
}  
 let performance = timer.elapsed() as f64 / size as f64;  
 println!("{size}: {performance} ns per push")  
}*  


The above programme takes an array with sizing values *100, 1000, 10000, 100000*, I am going to insert the values of the size n into a new array  
so a mutable array is declared. if the array is not mutable rust would not allow write permissions. *Note: the vector is not of fixed size*  
as with measuring performace we use the timer. The timer is refered from the library std::time::{instant}. the loop is startde for items in 0 to size  
and pushed to the mutable array. performance is calculate by dividing elapsed time as float by size as float 

** Observation: **
- Regardless of the size of the operation the insert into array was constant at around ~65 ns  
- except the first inert operation i.e 100, the rest all were constant   

** Inference: **   
*TBR-> When an operation takes the same time i.e The average constant time regardless of the size i.e 10x, 100x, 1000x  it is call amortized O(1).  
so why was the first operation slow? was it cos of the first initialisation? The real world analogy would be for the first time i took time to    
set up the jetson for time T, then for subsequent logins it was the same time t. if i take the average it would be amortized O(1)*  

**Review and Corrections**
- Time complexity is not the time taken for an agorithm takes to execute
- Rather time complexity describes how the computational work would grow as the size n of the input grows , it is analytical than arithmatic 
- The definition of Amortized O(1) while in itself is correct, the point where identified is wrong, the average time to insert an element inside an array is always constant
- If the array is of fixed size it is O(1), however if it is dynamic and it grows it is a gemetric series, growth events become less frequent 
- However the loop for n number of pushes scales linearily O(n)
- Also i am not using array rather i am using a dynamic vector, which brings to the observation taht rust would increase the size of the vector as it exceed inoit size on the fly 

***Result***
- Time complexity describes how computaional work grows as input size n grows, rather than measured execution time 
- The loop in the programme performs n calls to Vec::push, so the complexity is O(n)
- Vec::push is amortized O(1). Most pushes append into already allocated capcity and are O(1)
- When capacity is exhausted the vctor may allocate a larger contiguous region and move its existing elements, making that particular push O(n)
- Because capacity grows gemoetrically, these expensive reallocations become less frequent and the total reallocation work over n pushes remain O(n).
- Hence the average cost per push remains O(1)
- A Vec is a dynamic contiguous collection, not a fixed-size Rust array. It maintains a length and a capacity and may reallocate when its length exceeds the available capacity.
