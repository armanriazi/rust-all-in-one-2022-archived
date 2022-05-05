pub enum Tree<T> {
    LeafTail,
    Head(T,Box<Tree<T>>,Box<Tree<T>>),
}

use self::Tree::*;

impl <T:PartialOrd> Tree<T>{
    pub fn LeafTail()->Self{
        LeafTail
    }

    pub fn new(t:T)->Self{
        Head(t,Box::new(LeafTail),Box::new(LeafTail))
    }

    pub fn add(&mut self,t:T) {
        match self{
            LeafTail => {
                *self = Tree::new(t);
            },
            Head(d,lt,rt)=>{
                if t < *d {
                    lt.add(t);
                }else {
                    rt.add(t);
                }
            }
        }
    }
}

impl <T:Clone> Tree<T> {
    pub fn lt_d_rt(&self)->Vec<T>{
        match self {
            LeafTail => vec![],
            Head(d,lt,rt)=>{
                let mut res = lt.lt_d_rt();
                res.push(d.clone());
                res.append(&mut rt.lt_d_rt());
                res
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut t = Tree::new(5);
        t.add(4);
        t.add(3);
        t.add(9);
        assert_eq!(t.lt_d_rt(),vec![3,4,5,9]);
    }
}

/*

(t=4,d=5)         
(head={ 0 , d=5 )         
4-------------------------


(t=4)   
(leaftail={ 0 )   
(head={ 4 , leaftail,leaftail )      


(t=3,d=4)         
(head={ 0 , d=4 )         
3-------------------------


(t=3)   
(leaftail={ 0)   
(head={ 3 , leaftail,leaftail ) 


(t=9,d=5)         
(head={ 0 , d=5 )         
-------------------------9

(t=9)   
(leaftail={ 0)   
(head={ 9 , leaftail,leaftail ) 




res stack:
(head={ 5 , l ,r ) 
(head={ 4 , l ,r ) 
(head={ 3 , l ,r ) 
(head={ 9 , l ,r ) 

res.push 9
res.append  r

*/