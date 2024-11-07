// ReSharper disable InconsistentNaming
// ReSharper disable RedundantUnsafeContext
// ReSharper disable UnusedMember.Global
// ReSharper disable RedundantUsingDirective
using FyroxLite;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite;

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
    internal FeatureId value;
    internal int has_value;

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
internal partial struct FeatureId_slice
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

        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static FeatureId[]? _uploadBuffer;

    internal static FeatureId_slice FromFacade(in List<FeatureId> self)
    {
        _uploadBuffer ??= new FeatureId[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new FeatureId[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = __item;
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (FeatureId* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_fyrox_lite_lite_physics_LiteFeatureId_slice(new FeatureId_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial FeatureId_slice fyrox_lite_upload_fyrox_lite_lite_physics_LiteFeatureId_slice(FeatureId_slice managed);
}

[StructLayout(LayoutKind.Sequential)]
internal struct FeatureId_result
{
    internal int ok;
    internal FeatureId_result_value value;

    internal static unsafe FeatureId ToFacade(in FeatureId_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value.ok;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.value.err));
    }

    internal static FeatureId_result FromFacade(in FeatureId self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new FeatureId_result {ok = 1, value = new FeatureId_result_value { ok = __item_from_facade } };
    }

    internal static FeatureId_result FromFacadeError(in string err)
    {
        return new FeatureId_result {ok = 0, value = new FeatureId_result_value { err = NativeString.FromFacade(err) } };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct FeatureId_result_value
{
    [FieldOffset(0)]
    internal FeatureId ok;

    [FieldOffset(0)]
    internal NativeString err;
}