using FyroxLite.LiteInput;
using FyroxLite.LiteLog;
using FyroxLite.LiteMath;
using FyroxLite.LiteNode;
using FyroxLite.LitePhysics;
using FyroxLite.LitePlugin;
using FyroxLite.LitePrefab;
using FyroxLite.LiteScene;
using FyroxLite.LiteUi;
using FyroxLite.LiteWindow;
using System.Numerics;
using FyroxLite.LiteBase;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite.LitePhysics;

// fyrox_lite::lite_physics::LiteFeatureId
[StructLayout(LayoutKind.Sequential)]
public partial struct FeatureId
{
    public FeatureKind Kind {
        get => _kind;
        set => _kind = value;
    }
    public int Id {
        get => _id;
        set => _id = value;
    }
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private FeatureKind _kind;
    private int _id;
}

[StructLayout(LayoutKind.Sequential)]
internal struct FeatureId_optional
{
    private FeatureId value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static FeatureId? ToFacade(in FeatureId_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static FeatureId_optional FromFacade(in FeatureId? value)
    {
        if (value == null)
        {
            return new FeatureId_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new FeatureId_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct FeatureId_slice
{
    internal unsafe FeatureId* begin;
    internal int length;

    internal unsafe FeatureId_slice(FeatureId* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<FeatureId> ToFacade(in FeatureId_slice self)
    {
        var fetched = new List<FeatureId>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    internal static FeatureId_slice FromFacade(in List<FeatureId> self)
    {
        // __item
        throw new Exception("slice serialization not implemented yet");
    }

}

[StructLayout(LayoutKind.Explicit)]
internal struct FeatureId_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private FeatureId value;

    [FieldOffset(sizeof(int))]
    private string err;

    internal static unsafe FeatureId ToFacade(in FeatureId_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(self.err);
    }

    internal static FeatureId_result FromFacade(in FeatureId self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new FeatureId_result {ok = 1, value = __item_from_facade};
    }
}