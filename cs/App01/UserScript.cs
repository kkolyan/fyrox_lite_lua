using App01;

namespace FyroxLite.LiteBase;

internal readonly struct UserScript(long handle)
{
    private readonly long _handle = handle;

    internal static UserScript FromFacade(object value) => new(ObjectRegistry.Put(value));

    internal static object? ToFacade(UserScript id) => ObjectRegistry.Get(id._handle);

    internal static void DropMessage(UserScript id) => ObjectRegistry.Drop(id._handle);

    private bool Equals(UserScript other) => _handle == other._handle;

    public override bool Equals(object? obj) => obj is UserScript other && Equals(other);

    public override int GetHashCode() => _handle.GetHashCode();
}