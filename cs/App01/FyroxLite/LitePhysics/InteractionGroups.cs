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
using FyroxLite.LiteBase;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite.LitePhysics;

// fyrox_lite::lite_physics::LiteInteractionGroups
[StructLayout(LayoutKind.Sequential)]
public struct InteractionGroups
{
    public int Memberships {
        get => _memberships;
        set => _memberships = value;
    }
    public int Filter {
        get => _filter;
        set => _filter = value;
    }
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private int _memberships;
    private int _filter;
}

[StructLayout(LayoutKind.Sequential)]
internal struct InteractionGroups_optional
{
    internal InteractionGroups Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static InteractionGroups? ToFacade(in InteractionGroups_optional value)
    {
        if (value.HasValue)
        {
            var __item = value.Value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static InteractionGroups_optional FromFacade(in InteractionGroups? value)
    {
        if (value == null)
        {
            return new InteractionGroups_optional { Value = default, HasValue = false };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new InteractionGroups_optional { Value = __item_from_facade.Value, HasValue = true };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct InteractionGroups_slice
{
    private unsafe InteractionGroups* begin;
    private int length;
    internal List<InteractionGroups> Fetched;

    internal static unsafe void Fetch(ref InteractionGroups_slice self)
    {
        var fetched = new List<InteractionGroups>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        self.Fetched = fetched;
    }

    internal static unsafe List<InteractionGroups> ToFacade(in InteractionGroups_slice self)
    {
        var fetched = new List<InteractionGroups>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    internal static InteractionGroups_slice FromFacade(in List<InteractionGroups> self)
    {
        // __item
        throw new Exception("slice serialization not implemented yet");
    }

}

[StructLayout(LayoutKind.Explicit)]
internal struct InteractionGroups_result
{
    [FieldOffset(0)]
    internal int Ok;

    [FieldOffset(sizeof(int))]
    internal InteractionGroups Value;

    [FieldOffset(sizeof(int))]
    internal string Err;

    internal static unsafe InteractionGroups ToFacade(in InteractionGroups_result self)
    {
        if (self.Ok != 0)
        {
            var __item = self.Value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(self.Err);
    }

    internal static InteractionGroups_result FromFacade(in InteractionGroups self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new InteractionGroups_result {Ok = 1, Value = __item_from_facade};
    }
}