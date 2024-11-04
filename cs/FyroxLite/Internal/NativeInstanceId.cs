
namespace FyroxLite;

internal partial struct NativeInstanceId
{
    internal NativeInstanceId(long value)
    {
        this.value = value;
    }

    internal static NativeInstanceId FromFacade(object value)
    {
        if (value is long longValue)
        {
            throw new Exception("haha");
        }
        return new NativeInstanceId(ObjectRegistry.Put(value));
    }

    internal static object? ToFacade(NativeInstanceId id) => ObjectRegistry.Get(id.value);

    internal static void DropMessage(NativeInstanceId id) => ObjectRegistry.Drop(id.value);

    private bool Equals(NativeInstanceId other) => value == other.value;

    public override bool Equals(object? obj) => obj is NativeInstanceId other && Equals(other);

    public override int GetHashCode() => value.GetHashCode();
}