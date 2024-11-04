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
            _value ??= _byType.GetInRightThread()[typeof(T)];
            return _value.Value;
        }
    }

    [ThreadStatic] private static Dictionary<Type, NativeClassId>? _byType;
    [ThreadStatic] private static Dictionary<NativeClassId, Type>? _byId;

    internal Type GetCsClass()
    {
        if (_byId.GetInRightThread().TryGetValue(this, out var type))
        {
            return type;
        }

        throw new Exception($"No types associated with {this}. associations: [{string.Join(", ", _byId.GetInRightThread().Select(it => $"{it.Key}: {it.Value}"))}]");
    }

    internal static void InitThread()
    {
        _byType ??= new Dictionary<Type, NativeClassId>();
        _byId ??= new Dictionary<NativeClassId, Type>();
    }

    internal static void Register(Type type, NativeClassId id)
    {
        Console.WriteLine($"DEBUG C#: Associating {type.FullName} with {id}");
        _byType.GetInRightThread()[type] = id;
        _byId.GetInRightThread()[id] = type;
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

    public override string ToString()
    {
        return $"{nameof(NativeClassId)}({value})";
    }
}