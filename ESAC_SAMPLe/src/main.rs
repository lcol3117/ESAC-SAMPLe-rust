#[macro_use]
extern crate itertools;

use std::collections::HashSet;

fn main() {
  let gdata = vec![
    PLabelPoint::<bool> {
      label: None,
      point: vec![true, true, false, false, true]
    },
    PLabelPoint::<bool> {
      label: Some(LabelEnum::Safe),
      point: vec![true, false, false, false, true]
    },
    PLabelPoint::<bool> {
      label: Some(LabelEnum::Malware),
      point: vec![false, false, true, true, true]
    }
  ];
  let mut sample_model = ESAC::new(gdata);
  let n_clusters = sample_model.train();
  println!("{:?}", n_clusters);
  let result_ids = sample_model.cluster_ids;
  println!("{:?}", result_ids);
}

pub trait Distance where Self : std::marker::Sized {
  fn dist(a: Vec<Self>, b: Vec<Self>) -> u64;
}

pub trait Monoid where Self : std::marker::Sized {
  fn MonoidOperation(x: Self, y: Self) -> Self;
  fn MonoidIdentity() -> Self;
}

impl Distance for bool {
  fn dist(a: Vec<Self>, b: Vec<Self>) -> u64 {
    todo!()
  }
}

impl Monoid for bool {
  fn MonoidOperation(x: Self, y: Self) -> Self {
    x ^ y
  }
  fn MonoidIdentity() -> Self {
    false
  }
}

struct ESAC<T> {
  data: Vec<PLabelPoint<T>>,
  cluster_ids: Vec<u32>,
  exemplars: Vec<PLabelPoint<T>>,
  done: DoneEnum
}

impl<T : 
  std::clone::Clone +
  std::hash::Hash +
  Eq +
  PartialEq +
  std::cmp::Ord +
  std::cmp::PartialOrd +
  Monoid +
  Distance>
    ESAC<T> {
  fn new(data: Vec<PLabelPoint<T>>) -> Self {
    let empty_exemplars: Vec<PLabelPoint<T>> = vec![];
    let empty_cluster_ids: Vec<u32> = vec![];
    Self {
      data: data,
      cluster_ids: empty_cluster_ids,
      exemplars: empty_exemplars,
      done: DoneEnum::NotStarted
    }
  }
  fn train(&mut self) -> u32 {
    self.done = DoneEnum::NotDone;
    let label_pts = self.data
      .iter()
      .filter(|&x| {
        match (*x).label {
          Some(_label) => true,
          None => false
        }
      })
      .collect::<Vec<_>>();
    let label_max_dist = &label_pts
      .iter()
      .map(|x| {
        *(
          &label_pts
            .iter()
            .map(|y| {
              if (**x).label != (**y).label {
                T::dist(
                  (**x).point
                    .clone()
                    .to_vec(),
                  (**y).point
                    .clone()
                    .to_vec()
                )
              } else {
                0_u64
              }
            })
            .collect::<Vec<_>>()
            .clone()
            .iter()
            .fold(0_u64, |a, &v| {
              u64::max(a, v)
            })
        )
      })
      .collect::<Vec<_>>()
      .iter()
      .fold(0_u64, |a, &v| {
        u64::max(a, v)
      });
    self.exemplars = self.data
      .clone()
      .to_vec()
      .iter()
      .map(|x| { // v Placeholder
        PLabelPoint::<T> {
          label: Some(LabelEnum::Safe),
          point: vec![
            T::MonoidIdentity(),
            T::MonoidIdentity(),
            T::MonoidIdentity(),
            T::MonoidIdentity(),
            T::MonoidIdentity()
          ]
        }
      })
      .collect::<HashSet<_>>()
      .into_iter()
      .collect::<Vec<_>>();
    todo!();
    while self.done != DoneEnum::Done {
      for (a, b) in iproduct!(&label_pts, &label_pts) {
        if a.label != b.label {
          let index_a = &self.data
            .iter()
            .position(|x| {
              *x == **a
            })
            .unwrap();
          let index_b = &self.data
            .iter()
            .position(|x| {
              *x == **b
            })
            .unwrap();
          let id_a = (&self.cluster_ids)[*index_a];
          let id_b = (&self.cluster_ids)[*index_b];
          if id_a == id_b {
            self.done = DoneEnum::NotDone;
          }
        }
      }
      todo!();
    }
    *(
      &self.cluster_ids
        .iter()
        .cloned()
        .collect::<HashSet<_>>()
        .len()
    ) as u32
  }
}

#[derive(PartialEq, Eq, PartialOrd, Hash)]
struct PLabelPoint<T> {
  label: Option<LabelEnum>,
  point: Vec<T>
}

impl<T : std::clone::Clone> Clone for PLabelPoint<T> {
  fn clone(&self) -> Self {
    Self {
      label: self.label
        .clone(),
      point: (&self.point)
        .clone()
        .to_vec()
    }
  }
}

#[derive(PartialEq, Eq, PartialOrd, Hash)]
enum LabelEnum {
  Malware,
  Safe
}

impl Copy for LabelEnum {}

impl Clone for LabelEnum {
  fn clone(&self) -> Self {
    *self
  }
}

#[derive(PartialEq, Eq, PartialOrd, Hash)]
enum DoneEnum {
  NotStarted,
  NotDone,
  Done
}

impl Copy for DoneEnum {}

impl Clone for DoneEnum {
  fn clone(&self) -> Self {
    *self
  }
}