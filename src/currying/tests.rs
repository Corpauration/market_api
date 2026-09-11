use super::*;

#[test]
fn test_standard_workflow() {
    fn bind_values<Application: Bindable<frunk::HList![i32, i32]>>(application: Application) -> <Application as BindableExt<frunk::HList![i32, i32]>>::FullyApplied {
        application.bind(frunk::hlist![2, 3])
    }
    let add = |a: i32, b: i32| a + b;
    let fully_applied = bind_values(add.curry());
    let sum = fully_applied.eval();
    assert_eq!(sum, 5);
}

#[test]
fn test_stronger_bounds_for_concrete_associated_partials() {
    let add = |a: i32, b: i32| a + b;
    let curried_add = add.curry();
    let sum = curried_add.with(2).with(3).eval();
    assert_eq!(sum, 5);
}

#[test]
fn test_type_hell() {
    use std::collections::{ HashMap, HashSet, };
    use std::sync::{ Arc, Mutex, };
    use std::cell::{ RefCell, };
    use std::time::{ SystemTime, };

    fn hell<'a>(a: i32, b: String, c: (f32, f32), d: Arc<Mutex<Vec<i32>>>, e: Option<bool>, f: Result<(), String>, g: HashMap<String, i32>, h: Box<dyn Fn(i32) -> i32>, i: [u8; 16], j: &'a std::cell::RefCell<HashSet<SystemTime>>) -> (i32, String, (f32, f32), Arc<Mutex<Vec<i32>>>, Option<bool>, Result<(), String>, HashMap<String, i32>, Box<dyn Fn(i32) -> i32>, [u8; 16], &'a std::cell::RefCell<HashSet<SystemTime>>) {
        (a, b, c, d, e, f, g, h, i, j)
    }
    let h: Box<dyn Fn(i32) -> i32> = Box::new(|x: i32| x + 1);
    let j = RefCell::new(HashSet::new());
    let result = hell.curry()
        .with(42)
        .bind(frunk::hlist!["hello".to_string(), (3.14, 2.71), Arc::new(Mutex::new(vec![1, 2, 3]))])
        .with(Some(true))
        .bind(frunk::hlist![Ok(())])
        .bind(frunk::hlist![HashMap::new(), h, [0u8; 16]])
        .with(&j)
        .eval();
    assert_eq!(result.0, 42);
    assert_eq!(result.1, "hello");
    assert_eq!(result.2, (3.14, 2.71));
    assert_eq!(*result.3.lock().unwrap(), vec![1, 2, 3]);
    assert_eq!(result.4, Some(true));
    assert_eq!(result.5, Ok(()));
    assert_eq!(result.6, HashMap::new());
    assert_eq!((result.7)(1), 2);
    assert_eq!(result.8, [0u8; 16]);
    assert!(result.9.borrow().is_empty());
}

#[test]
fn test_sink()
{
    fn a<Application: Bindable<frunk::HList![i32]>>(application: Application) -> <Application as BindableExt<frunk::HList![i32]>>::FullyApplied {
        application.bind(frunk::hlist![2])
    }

    fn b<Application: Bindable<frunk::HList![i64, i64, i64]>>(application: Application) -> <Application as BindableExt<frunk::HList![i64, i64, i64]>>::FullyApplied {
        application.bind(frunk::hlist![2, 3, 4])
    }

    fn c<Application: Bindable<frunk::HList![f32, i16, bool]>>(application: Application) -> <Application as BindableExt<frunk::HList![f32, i16, bool]>>::FullyApplied {
        application.bind(frunk::hlist![3.14, 42, true])
    }

    let sink = Sink;
    let Sink = a(sink);
    let Sink = b(sink);
    let Sink = c(sink);
}

#[test]
fn test_curry_steps() {
    fn pack(a: i32, b: i32, c: i32, d: i32, e: i32) -> (i32, i32, i32, i32, i32) {
        (a, b, c, d, e)
    }

    #[rustfmt::skip]
    let result = pack.curry()
        .with(1)
        .curry_step::<frunk::HList![i32, i32]>()
            .with(2)
            .evaluate(3)
        .call(frunk::hlist![4, 5]);
    assert_eq!(result, (1, 2, 3, 4, 5));
}
