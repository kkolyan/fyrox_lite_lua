namespace FyroxLite;

internal static class PropertySetters
{
    public static void SetProperties(object obj, List<NativePropertyValue> state)
    {
        var properties = (_byType ?? throw new Exception("wrong thread"))[obj.GetType()] ??
                         throw new Exception($"unknown type: {obj.GetType()}");

        foreach (var propertyValue in state)
        {
            var propertyName = NativeString.ToFacade(propertyValue.name);
            var (type, setter) = properties[propertyName];
            if (type != propertyValue.ty)
            {
                Console.WriteLine(
                    $"Ignoring property deserialization due to type mismatch. property: {propertyName} expected type: {type}, incoming data type: {propertyValue.ty}");
                continue;
            }

            setter(obj, propertyValue.value);
        }
    }

    [ThreadStatic]
    private static Dictionary<Type, Dictionary<string, (NativeValueType, SetPropertyDelegate)>?>? _byType;

    internal static void Register(Type type, Dictionary<string, (NativeValueType, SetPropertyDelegate)> propertySetters)
    {
        _byType ??= new Dictionary<Type, Dictionary<string, (NativeValueType, SetPropertyDelegate)>>();
        _byType[type] = propertySetters;
    }


    internal delegate void SetPropertyDelegate(object obj, NativeValue value);
}