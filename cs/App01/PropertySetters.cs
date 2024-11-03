using FyroxLite.Internal;

namespace App01;

internal static class PropertySetters
{
    public static void SetProperty(object obj, int propertyId, NativeValue value)
    {
        (_byType ?? throw new Exception("wrong thread"))[obj.GetType()][propertyId](obj, value);
    }

    [ThreadStatic]
    private static Dictionary<Type, List<SetPropertyDelegate>?>? _byType;

    internal static void Register(Type type, List<SetPropertyDelegate> setPropertyList)
    {
        _byType ??= new Dictionary<Type, List<SetPropertyDelegate>>();
        _byType[type] = setPropertyList;
    }
    
    

    internal delegate void SetPropertyDelegate(object obj, NativeValue value);
}