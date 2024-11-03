namespace FyroxLite;

internal partial struct NativeClassId : IEquatable<NativeClassId>
{
    public NativeClassId(int value)
    {
        this.value = value;
    }

    internal static class By<T>
    {
        // ReSharper disable once StaticMemberInGenericType
        [ThreadStatic] private static NativeClassId? _value;

        internal static NativeClassId Resolve()
        {
            _value ??= (_byType ?? throw new Exception("wrong thread"))[typeof(T)];
            return _value.Value;
        }
    }

    [ThreadStatic] private static Dictionary<Type, NativeClassId>? _byType;
    [ThreadStatic] private static Dictionary<NativeClassId, Type>? _byId;

    internal Type GetType()
    {
        return (_byId ?? throw new Exception("wrong thread"))[this];
    }

    internal static void Register(Type type, NativeClassId id)
    {
        _byType ??= new Dictionary<Type, NativeClassId>();
        _byId ??= new Dictionary<NativeClassId, Type>();
        _byType[type] = id;
    }

    public bool Equals(NativeClassId other)
    {
        return value == other.value;
    }

    public override bool Equals(object? obj)
    {
        return obj is NativeClassId other && Equals(other);
    }

    public override int GetHashCode()
    {
        return value;
    }
}