// This file is generated and managed by `std_fn_once_implementations.rs.tt`.
// Any changes made directly to this file will be lost when the template is next run.

use super::*;

impl<Function, Result> StdFnOnce<frunk::HList![]> for Function
where
    Function: FnOnce() -> Result,
{
    type Result = Result;

    fn invoke(self, frunk::hlist_pat![]: frunk::HList![]) -> Self::Result {
        self()
    }
}

impl<Function, Param0, Result> StdFnOnce<frunk::HList![Param0]> for Function
where
    Function: FnOnce(Param0) -> Result,
{
    type Result = Result;

    fn invoke(self, frunk::hlist_pat![arg0]: frunk::HList![Param0]) -> Self::Result {
        self(arg0)
    }
}

impl<Function, Param0, Param1, Result> StdFnOnce<frunk::HList![Param0, Param1]> for Function
where
    Function: FnOnce(Param0, Param1) -> Result,
{
    type Result = Result;

    fn invoke(self, frunk::hlist_pat![arg0, arg1]: frunk::HList![Param0, Param1]) -> Self::Result {
        self(arg0, arg1)
    }
}

impl<Function, Param0, Param1, Param2, Result> StdFnOnce<frunk::HList![Param0, Param1, Param2]> for Function
where
    Function: FnOnce(Param0, Param1, Param2) -> Result,
{
    type Result = Result;

    fn invoke(self, frunk::hlist_pat![arg0, arg1, arg2]: frunk::HList![Param0, Param1, Param2]) -> Self::Result {
        self(arg0, arg1, arg2)
    }
}

impl<Function, Param0, Param1, Param2, Param3, Result> StdFnOnce<frunk::HList![Param0, Param1, Param2, Param3]> for Function
where
    Function: FnOnce(Param0, Param1, Param2, Param3) -> Result,
{
    type Result = Result;

    fn invoke(self, frunk::hlist_pat![arg0, arg1, arg2, arg3]: frunk::HList![Param0, Param1, Param2, Param3]) -> Self::Result {
        self(arg0, arg1, arg2, arg3)
    }
}

impl<Function, Param0, Param1, Param2, Param3, Param4, Result> StdFnOnce<frunk::HList![Param0, Param1, Param2, Param3, Param4]> for Function
where
    Function: FnOnce(Param0, Param1, Param2, Param3, Param4) -> Result,
{
    type Result = Result;

    fn invoke(self, frunk::hlist_pat![arg0, arg1, arg2, arg3, arg4]: frunk::HList![Param0, Param1, Param2, Param3, Param4]) -> Self::Result {
        self(arg0, arg1, arg2, arg3, arg4)
    }
}

impl<Function, Param0, Param1, Param2, Param3, Param4, Param5, Result> StdFnOnce<frunk::HList![Param0, Param1, Param2, Param3, Param4, Param5]> for Function
where
    Function: FnOnce(Param0, Param1, Param2, Param3, Param4, Param5) -> Result,
{
    type Result = Result;

    fn invoke(self, frunk::hlist_pat![arg0, arg1, arg2, arg3, arg4, arg5]: frunk::HList![Param0, Param1, Param2, Param3, Param4, Param5]) -> Self::Result {
        self(arg0, arg1, arg2, arg3, arg4, arg5)
    }
}

impl<Function, Param0, Param1, Param2, Param3, Param4, Param5, Param6, Result> StdFnOnce<frunk::HList![Param0, Param1, Param2, Param3, Param4, Param5, Param6]> for Function
where
    Function: FnOnce(Param0, Param1, Param2, Param3, Param4, Param5, Param6) -> Result,
{
    type Result = Result;

    fn invoke(self, frunk::hlist_pat![arg0, arg1, arg2, arg3, arg4, arg5, arg6]: frunk::HList![Param0, Param1, Param2, Param3, Param4, Param5, Param6]) -> Self::Result {
        self(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
    }
}

impl<Function, Param0, Param1, Param2, Param3, Param4, Param5, Param6, Param7, Result> StdFnOnce<frunk::HList![Param0, Param1, Param2, Param3, Param4, Param5, Param6, Param7]> for Function
where
    Function: FnOnce(Param0, Param1, Param2, Param3, Param4, Param5, Param6, Param7) -> Result,
{
    type Result = Result;

    fn invoke(self, frunk::hlist_pat![arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7]: frunk::HList![Param0, Param1, Param2, Param3, Param4, Param5, Param6, Param7]) -> Self::Result {
        self(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
    }
}

impl<Function, Param0, Param1, Param2, Param3, Param4, Param5, Param6, Param7, Param8, Result> StdFnOnce<frunk::HList![Param0, Param1, Param2, Param3, Param4, Param5, Param6, Param7, Param8]> for Function
where
    Function: FnOnce(Param0, Param1, Param2, Param3, Param4, Param5, Param6, Param7, Param8) -> Result,
{
    type Result = Result;

    fn invoke(self, frunk::hlist_pat![arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8]: frunk::HList![Param0, Param1, Param2, Param3, Param4, Param5, Param6, Param7, Param8]) -> Self::Result {
        self(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
    }
}

impl<Function, Param0, Param1, Param2, Param3, Param4, Param5, Param6, Param7, Param8, Param9, Result> StdFnOnce<frunk::HList![Param0, Param1, Param2, Param3, Param4, Param5, Param6, Param7, Param8, Param9]> for Function
where
    Function: FnOnce(Param0, Param1, Param2, Param3, Param4, Param5, Param6, Param7, Param8, Param9) -> Result,
{
    type Result = Result;

    fn invoke(self, frunk::hlist_pat![arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9]: frunk::HList![Param0, Param1, Param2, Param3, Param4, Param5, Param6, Param7, Param8, Param9]) -> Self::Result {
        self(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
    }
}

impl<Function, Param0, Param1, Param2, Param3, Param4, Param5, Param6, Param7, Param8, Param9, Param10, Result> StdFnOnce<frunk::HList![Param0, Param1, Param2, Param3, Param4, Param5, Param6, Param7, Param8, Param9, Param10]> for Function
where
    Function: FnOnce(Param0, Param1, Param2, Param3, Param4, Param5, Param6, Param7, Param8, Param9, Param10) -> Result,
{
    type Result = Result;

    fn invoke(self, frunk::hlist_pat![arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10]: frunk::HList![Param0, Param1, Param2, Param3, Param4, Param5, Param6, Param7, Param8, Param9, Param10]) -> Self::Result {
        self(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
    }
}

impl<Function, Param0, Param1, Param2, Param3, Param4, Param5, Param6, Param7, Param8, Param9, Param10, Param11, Result> StdFnOnce<frunk::HList![Param0, Param1, Param2, Param3, Param4, Param5, Param6, Param7, Param8, Param9, Param10, Param11]> for Function
where
    Function: FnOnce(Param0, Param1, Param2, Param3, Param4, Param5, Param6, Param7, Param8, Param9, Param10, Param11) -> Result,
{
    type Result = Result;

    fn invoke(self, frunk::hlist_pat![arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11]: frunk::HList![Param0, Param1, Param2, Param3, Param4, Param5, Param6, Param7, Param8, Param9, Param10, Param11]) -> Self::Result {
        self(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
    }
}


