
namespace FyroxLite;

internal partial struct NativeInstanceId(long handle)
{
    internal static NativeInstanceId FromFacade(object value) => new(ObjectRegistry.Put(value));

    internal static object? ToFacade(NativeInstanceId id) => ObjectRegistry.Get(id.value);

    internal static void DropMessage(NativeInstanceId id) => ObjectRegistry.Drop(id.value);

    private bool Equals(NativeInstanceId other) => value == other.value;

    public override bool Equals(object? obj) => obj is NativeInstanceId other && Equals(other);

    public override int GetHashCode() => value.GetHashCode();
}