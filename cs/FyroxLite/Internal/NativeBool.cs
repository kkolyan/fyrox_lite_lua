namespace FyroxLite.Internal;

internal partial struct NativeBool
{
    internal NativeBool(int value)
    {
        this.value = value;
    }

    public static bool ToFacade(NativeBool value)
    {
        return value.value != 0;
    }

    public static NativeBool FromFacade(bool value)
    {
        return new NativeBool(value ? 1 : 0);
    }
}