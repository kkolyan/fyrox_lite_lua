using App01;

namespace FyroxLite.LiteBase;

internal readonly struct UserScriptMessage(long handle) : IEquatable<UserScriptMessage>
{
    private readonly long _handle = handle;

    internal static UserScriptMessage FromFacade(object value) => new(ObjectRegistry.Put(value));

    internal static object? ToFacade(UserScriptMessage id) => ObjectRegistry.Get(id._handle);

    internal static void DropMessage(UserScriptMessage id) => ObjectRegistry.Drop(id._handle);

    public bool Equals(UserScriptMessage other) => _handle == other._handle;

    public override bool Equals(object? obj) => obj is UserScriptMessage other && Equals(other);

    public override int GetHashCode() => _handle.GetHashCode();
}