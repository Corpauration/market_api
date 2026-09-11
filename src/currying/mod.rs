use frunk::hlist::{ HCons, HNil, HList, };

/// Considering a type 'Augend' implementing `std::ops::Add` as an application 'f' from type to type mapping `RHS` to `Output` (not to be confused with `std::ops::Add::add`),
/// 'Augend' implementing `AddInto` should be an application 'g' from 'Im(f)' to the domain of 'f' mapping `Sum` to `Addend` such that 'g' is the inverse of 'f', when 'f' is injective.
/// In other words, where `<Self as std::ops::Add>` (read "'RHS |-> <Self as std::ops::Add<RHS>>::Output'") associates a sum type to the implementing augend type for a given addend type, `<Self as AddInto>` associates an addend type to the implementing augend type for a given sum type.
/// `<Augend as AddInto>` may be the inverse of a restriction of `<Augend as std::ops::Add>` to a canonical (read "semantically core (to the semantics of 'Augend')" as opposed to "convenience extensions") injective subdomain.
/// `<Augend as AddInto>` may be non-injectively extended to a non-canonical superdomain (*e.g.*, to support addition of a same addend to augend into distinct sums), in which case the restriction of `<Augend as AddInto>` to the canonical subdomain should remain the inverse of the restriction of `<Augend as std::ops::Add>` to the canonical subdomain.
/// For a type 'Augend' implementing `std::ops::Add` and `AddInto`, `<Augend as std::ops::Add>::add` and `<Augend as AddInto>::sum` should be equivalent on the canonical subdomain of '<Augend as std::ops::Add>' (*i.e.*, the codomain of the restriction of '<Augend as AddInto>' to the codomain of the restriction of '<Augend as std::ops::Add>' to the canonical subdomain).
pub trait AddInto<Sum> {
    type Addend;

    fn add_into(self, addend: Self::Addend) -> Sum;
}

impl<Sum: HList> AddInto<Sum> for HNil {
    type Addend = Sum;

    fn add_into(self, addend: Self::Addend) -> Sum {
        let HNil = self;
        addend
    }
}

impl<Head, Tail, TailSum> AddInto<HCons<Head, TailSum>> for HCons<Head, Tail>
where
    Tail: AddInto<TailSum>,
{
    type Addend = Tail::Addend;

    fn add_into(self, addend: Self::Addend) -> HCons<Head, TailSum> {
        HCons {
            head: self.head,
            tail: self.tail.add_into(addend),
        }
    }
}

pub trait AddIntoSelf: AddInto<Self> + Sized {}
impl<This: AddInto<Self>> AddIntoSelf for This {}

pub trait Bindable<Signature> {
    type Partial<Bound: AddInto<Signature>>: Bindable<Bound::Addend>;

    fn bind<Arguments: AddInto<Signature>>(self, arguments: Arguments) -> Self::Partial<Arguments>;
}

pub trait BindableExt<Signature>: Bindable<Signature> {
    fn with<Argument>(self, argument: Argument) -> Self::Partial<frunk::HList![Argument]>
    where
        frunk::HList![Argument]: AddInto<Signature>,
    ;

    fn bind_with_default(self) -> Self::Partial<Signature>
    where
        Signature: Default + AddIntoSelf,
    ;

    type FullyApplied: Bindable<Signature::Addend>
    where
        Signature: AddIntoSelf
    ;
}
impl<Signature, This: Bindable<Signature>> BindableExt<Signature> for This {
    fn with<Argument>(self, argument: Argument) -> Self::Partial<frunk::HList![Argument]>
    where
        frunk::HList![Argument]: AddInto<Signature>,
    {
        self.bind(frunk::hlist![argument])
    }

    fn bind_with_default(self) -> Self::Partial<Signature>
    where
        Signature: Default + AddIntoSelf,
    {
        self.bind(Signature::default())
    }

    type FullyApplied = Self::Partial<Signature>
    where
        Signature: AddIntoSelf
    ;
}

pub trait Callable<Signature> {
    type Result;

    fn call(self, arguments: Signature) -> Self::Result;
}

pub trait CallableExt<Signature>: Callable<Signature> {
    fn call_with_default(self) -> Self::Result
    where
        Signature: Default,
    ;

    fn eval(self) -> Self::Result
    where
        Signature: From<()>,
    ;

    fn curry(self) -> Curry<Self>
    where
        Self: Sized,
    ;
}
impl<Signature, This: Callable<Signature>> CallableExt<Signature> for This {
    fn call_with_default(self) -> Self::Result
    where
        Signature: Default,
    {
        self.call(Signature::default())
    }

    fn eval(self) -> Self::Result
    where
        Signature: From<()>,
    {
        self.call(Signature::from(()))
    }

    fn curry(self) -> Curry<Self>
    where
        Self: Sized,
    {
        Curry(self)
    }
}

pub trait Operation<Signature>: Callable<Signature> + Bindable<Signature>
where
    // As of Rust 1.98.1, the following bound is not yet supported in stable Rust.
    // This is worked around with `Self::PartialOperation` and `Self::partial_is_partial_operation`.
    /*
    for<Bound: AddInto<Signature>> Self::Partial<Bound>: Operation<Bound::Addend>
    */
{
    type PartialOperation<Bound: AddInto<Signature>>: Operation<Bound::Addend>;
    fn partial_is_partial_operation<Bound: AddInto<Signature>>(partial: Self::Partial<Bound>) -> Self::PartialOperation<Bound>;
}
pub use Operation as Op;

pub trait OperationExt<Signature>: Operation<Signature> {
    fn op_bind<Arguments: AddInto<Signature>>(self, arguments: Arguments) -> Self::PartialOperation<Arguments>;

    fn op_with<Argument>(self, argument: Argument) -> Self::PartialOperation<frunk::HList![Argument]>
    where
        frunk::HList![Argument]: AddInto<Signature>,
    ;

    fn op_bind_with_default(self) -> Self::PartialOperation<Signature>
    where
        Signature: Default + AddIntoSelf,
    ;

    type FullyAppliedOp: Operation<Signature::Addend>
    where
        Signature: AddIntoSelf
    ;
}
impl<Signature, This: Operation<Signature>> OperationExt<Signature> for This {
    fn op_bind<Arguments: AddInto<Signature>>(self, arguments: Arguments) -> Self::PartialOperation<Arguments> {
        Self::partial_is_partial_operation(self.bind(arguments))
    }

    fn op_with<Argument>(self, argument: Argument) -> Self::PartialOperation<frunk::HList![Argument]>
    where
        frunk::HList![Argument]: AddInto<Signature>,
    {
        Self::partial_is_partial_operation(self.with(argument))
    }

    fn op_bind_with_default(self) -> Self::PartialOperation<Signature>
    where
        Signature: Default + AddIntoSelf,
    {
        Self::partial_is_partial_operation(self.bind_with_default())
    }

    type FullyAppliedOp = Self::PartialOperation<Signature>
    where
        Signature: AddIntoSelf
    ;
}

#[rustfmt::skip]
#[derive(Debug)] #[derive(Clone, Copy)] #[derive(PartialEq, Eq)] #[derive(PartialOrd, Ord)] #[derive(Hash)]
#[derive(frunk::Generic, frunk::LabelledGeneric)]
pub struct Folder<Head, Tail> {
    pub head: Head,
    pub tail: Tail,
}

impl<Head, Tail, Sum> AddInto<Sum> for Folder<Head, Tail>
where
    Head: AddInto<Sum>,
    Tail: AddInto<Head::Addend>,
{
    type Addend = Tail::Addend;

    fn add_into(self, addend: Self::Addend) -> Sum {
        let Folder { head, tail } = self;
        head.add_into(tail.add_into(addend))
    }
}

#[rustfmt::skip]
#[derive(Debug)] #[derive(Clone, Copy)] #[derive(PartialEq, Eq)] #[derive(Hash)]
pub struct Partial<Application, PartialArguments, Signature> {
    pub application: Application,
    pub partial_arguments: PartialArguments,
    pub _signature: std::marker::PhantomData<Signature>,
}

impl<Application, PartialArguments, Signature> Partial<Application, PartialArguments, Signature> {
    pub fn new(application: Application, partial_arguments: PartialArguments) -> Self {
        Self {
            application,
            partial_arguments,
            _signature: std::marker::PhantomData,
        }
    }
}

impl<Application, PartialArguments, Signature> Bindable<PartialArguments::Addend> for Partial<Application, PartialArguments, Signature>
where
    Application: Bindable<Signature>,
    PartialArguments: AddInto<Signature>,
{
    type Partial<Bound: AddInto<PartialArguments::Addend>> = Partial<Application, Folder<PartialArguments, Bound>, Signature>;

    fn bind<Arguments: AddInto<PartialArguments::Addend>>(self, arguments: Arguments) -> Self::Partial<Arguments> {
        let Partial { application, partial_arguments, .. } = self;
        Partial::new(application, Folder { head: partial_arguments, tail: arguments })
    }
}

impl<Application, PartialArguments, Signature> Callable<PartialArguments::Addend> for Partial<Application, PartialArguments, Signature>
where
    Application: Callable<Signature>,
    PartialArguments: AddInto<Signature>,
{
    type Result = Application::Result;

    fn call(self, arguments: PartialArguments::Addend) -> Self::Result {
        let Partial { application, partial_arguments, .. } = self;
        application.call(partial_arguments.add_into(arguments))
    }
}

impl<Application, PartialArguments, Signature> Operation<PartialArguments::Addend> for Partial<Application, PartialArguments, Signature>
where
    Application: Operation<Signature>,
    PartialArguments: AddInto<Signature>,
{
    type PartialOperation<Bound: AddInto<PartialArguments::Addend>> = Self::Partial<Bound>;

    fn partial_is_partial_operation<Bound: AddInto<PartialArguments::Addend>>(partial: Self::Partial<Bound>) -> Self::PartialOperation<Bound> {
        partial
    }
}

#[rustfmt::skip]
#[derive(Debug)] #[derive(Clone, Copy)] #[derive(PartialEq, Eq)] #[derive(PartialOrd, Ord)] #[derive(Hash)] #[derive(Default)]
#[derive(frunk::Generic, frunk::LabelledGeneric)]
pub struct Sink;

impl<Signature> Bindable<Signature> for Sink {
    type Partial<Bound: AddInto<Signature>> = Sink;

    fn bind<Arguments: AddInto<Signature>>(self, arguments: Arguments) -> Self::Partial<Arguments> {
        _ = arguments;
        Sink
    }
}

#[rustfmt::skip]
#[derive(Debug)] #[derive(Clone, Copy)] #[derive(PartialEq, Eq)] #[derive(PartialOrd, Ord)] #[derive(Hash)] #[derive(Default)]
#[derive(frunk::Generic, frunk::LabelledGeneric)]
pub struct Accumulator;

impl<Signature> Bindable<Signature> for Accumulator {
    type Partial<Bound: AddInto<Signature>> = Partial<Accumulator, Bound, Signature>;

    fn bind<Arguments: AddInto<Signature>>(self, arguments: Arguments) -> Self::Partial<Arguments> {
        Partial::new(self, arguments)
    }
}

#[rustfmt::skip]
#[derive(Debug)] #[derive(Clone, Copy)] #[derive(PartialEq, Eq)] #[derive(PartialOrd, Ord)] #[derive(Hash)] #[derive(Default)]
#[derive(frunk::Generic, frunk::LabelledGeneric)]
pub struct Curry<Application>(pub Application);

impl<Application, Signature> Bindable<Signature> for Curry<Application>
{
    type Partial<Bound: AddInto<Signature>> = Partial<Curry<Application>, Bound, Signature>;

    fn bind<Arguments: AddInto<Signature>>(self, arguments: Arguments) -> Self::Partial<Arguments> {
        Partial::new(self, arguments)
    }
}

impl<Application, Signature> Callable<Signature> for Curry<Application>
where
    Application: Callable<Signature>,
{
    type Result = Application::Result;

    fn call(self, arguments: Signature) -> Self::Result {
        let Curry(application) = self;
        application.call(arguments)
    }
}

impl<Application, Signature> Operation<Signature> for Curry<Application>
where
    Application: Operation<Signature>,
{
    type PartialOperation<Bound: AddInto<Signature>> = Partial<Curry<Application>, Bound, Signature>;

    fn partial_is_partial_operation<Bound: AddInto<Signature>>(partial: Self::Partial<Bound>) -> Self::PartialOperation<Bound> {
        partial
    }
}

pub trait StdFnOnce<Signature>
{
    type Result;

    fn invoke(self, arguments: Signature) -> Self::Result;
}

#[doc(hidden)]
mod std_fn_once_implementations;
#[allow(unused_imports)]
pub use std_fn_once_implementations::*;

pub trait StdFnOnceExt<Signature>: StdFnOnce<Signature> {
    fn curry(self) -> Curry<Func<Self>>
    where
        Self: Sized,
    ;
}
impl<Signature, This: StdFnOnce<Signature>> StdFnOnceExt<Signature> for This {
    fn curry(self) -> Curry<Func<Self>>
    where
        Self: Sized,
    {
        Curry(Func(self))
    }
}

#[rustfmt::skip]
#[derive(Debug)] #[derive(Clone, Copy)] #[derive(PartialEq, Eq)] #[derive(PartialOrd, Ord)] #[derive(Hash)] #[derive(Default)]
#[derive(frunk::Generic, frunk::LabelledGeneric)]
pub struct Func<Function>(pub Function);

impl<Function, Signature> Bindable<Signature> for Func<Function>
where
    Function: StdFnOnce<Signature>,
{
    type Partial<Bound: AddInto<Signature>> = Partial<Func<Function>, Bound, Signature>;

    fn bind<Arguments: AddInto<Signature>>(self, arguments: Arguments) -> Self::Partial<Arguments> {
        Partial::new(self, arguments)
    }
}

impl<Function, Signature> Callable<Signature> for Func<Function>
where
    Function: StdFnOnce<Signature>,
{
    type Result = Function::Result;

    fn call(self, arguments: Signature) -> Self::Result {
        let Func(function) = self;
        function.invoke(arguments)
    }
}

impl<Function, Signature> Operation<Signature> for Func<Function>
where
    Function: StdFnOnce<Signature>,
{
    type PartialOperation<Bound: AddInto<Signature>> = Partial<Func<Function>, Bound, Signature>;

    fn partial_is_partial_operation<Bound: AddInto<Signature>>(partial: Self::Partial<Bound>) -> Self::PartialOperation<Bound> {
        partial
    }
}

#[cfg(test)]
mod tests;