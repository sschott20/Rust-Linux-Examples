# automatically generated by the FlatBuffers compiler, do not modify

# namespace: tflite

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class QuantizationParameters(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = QuantizationParameters()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsQuantizationParameters(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    @classmethod
    def QuantizationParametersBufferHasIdentifier(cls, buf, offset, size_prefixed=False):
        return flatbuffers.util.BufferHasIdentifier(buf, offset, b"\x54\x46\x4C\x33", size_prefixed=size_prefixed)

    # QuantizationParameters
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # QuantizationParameters
    def Min(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Float32Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 4))
        return 0

    # QuantizationParameters
    def MinAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Float32Flags, o)
        return 0

    # QuantizationParameters
    def MinLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # QuantizationParameters
    def MinIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        return o == 0

    # QuantizationParameters
    def Max(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Float32Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 4))
        return 0

    # QuantizationParameters
    def MaxAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Float32Flags, o)
        return 0

    # QuantizationParameters
    def MaxLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # QuantizationParameters
    def MaxIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        return o == 0

    # QuantizationParameters
    def Scale(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Float32Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 4))
        return 0

    # QuantizationParameters
    def ScaleAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Float32Flags, o)
        return 0

    # QuantizationParameters
    def ScaleLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # QuantizationParameters
    def ScaleIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        return o == 0

    # QuantizationParameters
    def ZeroPoint(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int64Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 8))
        return 0

    # QuantizationParameters
    def ZeroPointAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int64Flags, o)
        return 0

    # QuantizationParameters
    def ZeroPointLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # QuantizationParameters
    def ZeroPointIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        return o == 0

    # QuantizationParameters
    def DetailsType(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Uint8Flags, o + self._tab.Pos)
        return 0

    # QuantizationParameters
    def Details(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(14))
        if o != 0:
            from flatbuffers.table import Table
            obj = Table(bytearray(), 0)
            self._tab.Union(obj, o)
            return obj
        return None

    # QuantizationParameters
    def QuantizedDimension(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(16))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

def QuantizationParametersStart(builder):
    builder.StartObject(7)

def Start(builder):
    QuantizationParametersStart(builder)

def QuantizationParametersAddMin(builder, min):
    builder.PrependUOffsetTRelativeSlot(0, flatbuffers.number_types.UOffsetTFlags.py_type(min), 0)

def AddMin(builder, min):
    QuantizationParametersAddMin(builder, min)

def QuantizationParametersStartMinVector(builder, numElems):
    return builder.StartVector(4, numElems, 4)

def StartMinVector(builder, numElems: int) -> int:
    return QuantizationParametersStartMinVector(builder, numElems)

def QuantizationParametersAddMax(builder, max):
    builder.PrependUOffsetTRelativeSlot(1, flatbuffers.number_types.UOffsetTFlags.py_type(max), 0)

def AddMax(builder, max):
    QuantizationParametersAddMax(builder, max)

def QuantizationParametersStartMaxVector(builder, numElems):
    return builder.StartVector(4, numElems, 4)

def StartMaxVector(builder, numElems: int) -> int:
    return QuantizationParametersStartMaxVector(builder, numElems)

def QuantizationParametersAddScale(builder, scale):
    builder.PrependUOffsetTRelativeSlot(2, flatbuffers.number_types.UOffsetTFlags.py_type(scale), 0)

def AddScale(builder, scale):
    QuantizationParametersAddScale(builder, scale)

def QuantizationParametersStartScaleVector(builder, numElems):
    return builder.StartVector(4, numElems, 4)

def StartScaleVector(builder, numElems: int) -> int:
    return QuantizationParametersStartScaleVector(builder, numElems)

def QuantizationParametersAddZeroPoint(builder, zeroPoint):
    builder.PrependUOffsetTRelativeSlot(3, flatbuffers.number_types.UOffsetTFlags.py_type(zeroPoint), 0)

def AddZeroPoint(builder, zeroPoint):
    QuantizationParametersAddZeroPoint(builder, zeroPoint)

def QuantizationParametersStartZeroPointVector(builder, numElems):
    return builder.StartVector(8, numElems, 8)

def StartZeroPointVector(builder, numElems: int) -> int:
    return QuantizationParametersStartZeroPointVector(builder, numElems)

def QuantizationParametersAddDetailsType(builder, detailsType):
    builder.PrependUint8Slot(4, detailsType, 0)

def AddDetailsType(builder, detailsType):
    QuantizationParametersAddDetailsType(builder, detailsType)

def QuantizationParametersAddDetails(builder, details):
    builder.PrependUOffsetTRelativeSlot(5, flatbuffers.number_types.UOffsetTFlags.py_type(details), 0)

def AddDetails(builder, details):
    QuantizationParametersAddDetails(builder, details)

def QuantizationParametersAddQuantizedDimension(builder, quantizedDimension):
    builder.PrependInt32Slot(6, quantizedDimension, 0)

def AddQuantizedDimension(builder, quantizedDimension):
    QuantizationParametersAddQuantizedDimension(builder, quantizedDimension)

def QuantizationParametersEnd(builder):
    return builder.EndObject()

def End(builder):
    return QuantizationParametersEnd(builder)

import tflite.CustomQuantization
import tflite.QuantizationDetails
try:
    from typing import List, Union
except:
    pass

class QuantizationParametersT(object):

    # QuantizationParametersT
    def __init__(self):
        self.min = None  # type: List[float]
        self.max = None  # type: List[float]
        self.scale = None  # type: List[float]
        self.zeroPoint = None  # type: List[int]
        self.detailsType = 0  # type: int
        self.details = None  # type: Union[None, tflite.CustomQuantization.CustomQuantizationT]
        self.quantizedDimension = 0  # type: int

    @classmethod
    def InitFromBuf(cls, buf, pos):
        quantizationParameters = QuantizationParameters()
        quantizationParameters.Init(buf, pos)
        return cls.InitFromObj(quantizationParameters)

    @classmethod
    def InitFromPackedBuf(cls, buf, pos=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, pos)
        return cls.InitFromBuf(buf, pos+n)

    @classmethod
    def InitFromObj(cls, quantizationParameters):
        x = QuantizationParametersT()
        x._UnPack(quantizationParameters)
        return x

    # QuantizationParametersT
    def _UnPack(self, quantizationParameters):
        if quantizationParameters is None:
            return
        if not quantizationParameters.MinIsNone():
            if np is None:
                self.min = []
                for i in range(quantizationParameters.MinLength()):
                    self.min.append(quantizationParameters.Min(i))
            else:
                self.min = quantizationParameters.MinAsNumpy()
        if not quantizationParameters.MaxIsNone():
            if np is None:
                self.max = []
                for i in range(quantizationParameters.MaxLength()):
                    self.max.append(quantizationParameters.Max(i))
            else:
                self.max = quantizationParameters.MaxAsNumpy()
        if not quantizationParameters.ScaleIsNone():
            if np is None:
                self.scale = []
                for i in range(quantizationParameters.ScaleLength()):
                    self.scale.append(quantizationParameters.Scale(i))
            else:
                self.scale = quantizationParameters.ScaleAsNumpy()
        if not quantizationParameters.ZeroPointIsNone():
            if np is None:
                self.zeroPoint = []
                for i in range(quantizationParameters.ZeroPointLength()):
                    self.zeroPoint.append(quantizationParameters.ZeroPoint(i))
            else:
                self.zeroPoint = quantizationParameters.ZeroPointAsNumpy()
        self.detailsType = quantizationParameters.DetailsType()
        self.details = tflite.QuantizationDetails.QuantizationDetailsCreator(self.detailsType, quantizationParameters.Details())
        self.quantizedDimension = quantizationParameters.QuantizedDimension()

    # QuantizationParametersT
    def Pack(self, builder):
        if self.min is not None:
            if np is not None and type(self.min) is np.ndarray:
                min = builder.CreateNumpyVector(self.min)
            else:
                QuantizationParametersStartMinVector(builder, len(self.min))
                for i in reversed(range(len(self.min))):
                    builder.PrependFloat32(self.min[i])
                min = builder.EndVector()
        if self.max is not None:
            if np is not None and type(self.max) is np.ndarray:
                max = builder.CreateNumpyVector(self.max)
            else:
                QuantizationParametersStartMaxVector(builder, len(self.max))
                for i in reversed(range(len(self.max))):
                    builder.PrependFloat32(self.max[i])
                max = builder.EndVector()
        if self.scale is not None:
            if np is not None and type(self.scale) is np.ndarray:
                scale = builder.CreateNumpyVector(self.scale)
            else:
                QuantizationParametersStartScaleVector(builder, len(self.scale))
                for i in reversed(range(len(self.scale))):
                    builder.PrependFloat32(self.scale[i])
                scale = builder.EndVector()
        if self.zeroPoint is not None:
            if np is not None and type(self.zeroPoint) is np.ndarray:
                zeroPoint = builder.CreateNumpyVector(self.zeroPoint)
            else:
                QuantizationParametersStartZeroPointVector(builder, len(self.zeroPoint))
                for i in reversed(range(len(self.zeroPoint))):
                    builder.PrependInt64(self.zeroPoint[i])
                zeroPoint = builder.EndVector()
        if self.details is not None:
            details = self.details.Pack(builder)
        QuantizationParametersStart(builder)
        if self.min is not None:
            QuantizationParametersAddMin(builder, min)
        if self.max is not None:
            QuantizationParametersAddMax(builder, max)
        if self.scale is not None:
            QuantizationParametersAddScale(builder, scale)
        if self.zeroPoint is not None:
            QuantizationParametersAddZeroPoint(builder, zeroPoint)
        QuantizationParametersAddDetailsType(builder, self.detailsType)
        if self.details is not None:
            QuantizationParametersAddDetails(builder, details)
        QuantizationParametersAddQuantizedDimension(builder, self.quantizedDimension)
        quantizationParameters = QuantizationParametersEnd(builder)
        return quantizationParameters
