namespace FyroxLite;

internal partial struct UserScriptMessage
{
    private UserScriptMessage(long id)
    {
        this.id = id;
    }

    internal static UserScriptMessage FromFacade(object value) => new(ObjectRegistry.Put(value));

    internal static object? ToFacade(UserScriptMessage id) => ObjectRegistry.Get(id.id);

    internal static void DropMessage(UserScriptMessage id) => ObjectRegistry.Drop(id.id);

    public bool Equals(UserScriptMessage other) => id == other.id;

    public override bool Equals(object? obj) => obj is UserScriptMessage other && Equals(other);

    public override int GetHashCode() => id.GetHashCode();
}