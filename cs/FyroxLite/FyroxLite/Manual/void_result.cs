using System.Runtime.InteropServices;

namespace FyroxLite;

[StructLayout(LayoutKind.Sequential)]
internal struct void_result
{
    internal int ok;
    internal NativeString err;

    internal static void ToFacade(in void_result self)
    {
        if (self.ok != 0)
        {
            return;
        }
        throw new Exception(NativeString.ToFacade(self.err));
    }

    internal static void_result FromFacade()
    {
        return new void_result {ok = 1 };
    }

    internal static void_result FromFacadeError(in string err)
    {
        return new void_result {ok = 0, err = NativeString.FromFacade(err) };
    }
}