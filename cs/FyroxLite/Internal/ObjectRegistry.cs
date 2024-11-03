namespace FyroxLite;

internal static class ObjectRegistry
{
    [ThreadStatic]
    private static Dictionary<long, object>? _objects;

    private static long _nextId;

    internal static long Put(object value)
    {
        _objects ??= new Dictionary<long, object>();
        var id = Interlocked.Increment(ref _nextId);
        _objects.Add(id, value);
        return id;
    }

    internal static object? Get(long id)
    {
        return _objects?.TryGetValue(id, out var value) == true ? value : null;
    }

    internal static void Drop(long id)
    {
        _objects?.Remove(id);
    }
}