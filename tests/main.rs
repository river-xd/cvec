use std::{
  ffi::c_void,
  mem::{self, size_of}
};

type Destructor=extern "C" fn(&c_void);

#[repr(C)]
struct Vec {
  bytes_per_element: usize,
  len: usize,
  capacity: usize,
  ptr: *mut c_void,
  destructor: Destructor,
}

#[link(name="cvec",kind="static")]
#[allow(dead_code)]
extern "C" {
  fn new_vec(bytes_per_element: usize,destructor: Destructor)-> Vec;
  fn new_vec_with_capacity(capacity: usize,bytes_per_element: usize,destructor: Destructor)-> Vec;
  fn drop_vec(this: &Vec);
  fn vec_push(this: &Vec,element: *mut c_void);
  fn drop_pop(this: &Vec);
  fn vec_reserve(this: &Vec,additional: usize);
  fn vec_reserve_exact(this: &Vec,additional: usize);
  fn vec_append(this: &Vec,other: *mut c_void);
  fn vec_extend(this: &Vec,data: *mut c_void,len: usize);
  fn vec_clear(this: &Vec);
  fn vec_insert(this: &Vec,index: usize,element: *mut c_void);
  fn vec_remove(this: &Vec,index: usize)-> *mut c_void;
  fn vec_truncate(this: &Vec,len: usize);
  fn vec_resize(this: &Vec,new_len: usize,value: *mut c_void);
  fn vec_resize_with(this: &Vec,new_len: usize,f: extern "C" fn(*mut c_void)-> *mut c_void);
  fn vec_retain(this: &Vec,f: extern "C" fn(*mut i32)-> bool);
}



fn main() {
  unsafe {
    _main();
  }
}

#[allow(invalid_value)]
unsafe fn _main() {
  let vec=new_vec(size_of::<i32>(),mem::transmute(0usize));

  fill_vec(&vec);
  vec_retain(&vec,is_even);

  let arr=vec.ptr as *mut i32;
  for i in 0..vec.len {
    print!("{},",*arr.add(i));
  }
  println!();

  drop_vec(&vec);
}

#[allow(dead_code)]
unsafe fn print_vec(this: &Vec,idx: usize) {
  print!("{idx}: ");
  for i in std::slice::from_raw_parts(this.ptr as *mut i32,this.len) {
    print!("{i} ");
  }
  println!();
}

unsafe fn fill_vec(this: &Vec) {
  for mut i in 0..32 {
    vec_push(this,&mut i as *mut i32 as *mut _);
  }
}

extern "C" fn is_even(ptr: *mut i32)-> bool {
  unsafe {
    *ptr%2==0
  }
}

