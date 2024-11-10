namespace FyroxLite;

internal static class ObjectRegistry
{
    [ThreadStatic]
    private static Dictionary<long, object>? _objects;

    private static long _nextId;

    internal static void InitThread()
    {
        _objects ??= new Dictionary<long, object>();
    }

    internal static long Put(object value)
    {
        var id = Interlocked.Increment(ref _nextId);
        _objects.GetInRightThread().Add(id, value);
        return id;
    }

    internal static object? Get(long id)
    {
        return _objects.GetInRightThread().TryGetValue(id, out var value) ? value : null;
    }

    internal static void Drop(long id)
    {
        _objects.GetInRightThread().Remove(id);
    }
}