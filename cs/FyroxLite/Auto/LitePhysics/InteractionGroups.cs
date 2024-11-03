// ReSharper disable InconsistentNaming
// ReSharper disable RedundantUnsafeContext
// ReSharper disable UnusedMember.Global
// ReSharper disable RedundantUsingDirective
using FyroxLite.Internal;
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
using System.Drawing;
using FyroxLite.Internal;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite.LitePhysics;

// fyrox_lite::lite_physics::LiteInteractionGroups
[StructLayout(LayoutKind.Sequential)]
public partial struct InteractionGroups
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
    private InteractionGroups value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static InteractionGroups? ToFacade(in InteractionGroups_optional value)
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
    public static InteractionGroups_optional FromFacade(in InteractionGroups? value)
    {
        if (value == null)
        {
            return new InteractionGroups_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new InteractionGroups_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct InteractionGroups_slice
{
    internal unsafe InteractionGroups* begin;
    internal int length;

    internal unsafe InteractionGroups_slice(InteractionGroups* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<InteractionGroups> ToFacade(in InteractionGroups_slice self)
    {
        var fetched = new List<InteractionGroups>();

        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static InteractionGroups[]? _uploadBuffer;

    internal static InteractionGroups_slice FromFacade(in List<InteractionGroups> self)
    {
        _uploadBuffer ??= new InteractionGroups[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new InteractionGroups[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = __item;
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (InteractionGroups* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_fyrox_lite_lite_physics_LiteInteractionGroups_slice(new InteractionGroups_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial InteractionGroups_slice fyrox_lite_upload_fyrox_lite_lite_physics_LiteInteractionGroups_slice(InteractionGroups_slice managed);
}

[StructLayout(LayoutKind.Explicit)]
internal struct InteractionGroups_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private InteractionGroups value;

    [FieldOffset(sizeof(int))]
    private NativeString err;

    internal static unsafe InteractionGroups ToFacade(in InteractionGroups_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.err));
    }

    internal static InteractionGroups_result FromFacade(in InteractionGroups self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new InteractionGroups_result {ok = 1, value = __item_from_facade};
    }

    internal static InteractionGroups_result FromFacadeError(in string err)
    {
        return new InteractionGroups_result {ok = 0, err = NativeString.FromFacade(err)};
    }
}