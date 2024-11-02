using System.Runtime.InteropServices;

namespace FyroxLite;

[StructLayout(LayoutKind.Sequential)]
internal readonly struct NativeBool
{
    private readonly int _value;

    internal NativeBool(int value)
    {
        _value = value;
    }

    public static bool ToFacade(NativeBool value)
    {
        return value._value != 0;
    }

    public static NativeBool FromFacade(bool value)
    {
        return new NativeBool(value ? 1 : 0);
    }
}