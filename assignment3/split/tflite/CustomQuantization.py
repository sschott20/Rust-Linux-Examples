# automatically generated by the FlatBuffers compiler, do not modify

# namespace: tflite

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class CustomQuantization(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = CustomQuantization()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsCustomQuantization(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    @classmethod
    def CustomQuantizationBufferHasIdentifier(cls, buf, offset, size_prefixed=False):
        return flatbuffers.util.BufferHasIdentifier(buf, offset, b"\x54\x46\x4C\x33", size_prefixed=size_prefixed)

    # CustomQuantization
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # CustomQuantization
    def Custom(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Uint8Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 1))
        return 0

    # CustomQuantization
    def CustomAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Uint8Flags, o)
        return 0

    # CustomQuantization
    def CustomLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # CustomQuantization
    def CustomIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        return o == 0

def CustomQuantizationStart(builder):
    builder.StartObject(1)

def Start(builder):
    CustomQuantizationStart(builder)

def CustomQuantizationAddCustom(builder, custom):
    builder.PrependUOffsetTRelativeSlot(0, flatbuffers.number_types.UOffsetTFlags.py_type(custom), 0)

def AddCustom(builder, custom):
    CustomQuantizationAddCustom(builder, custom)

def CustomQuantizationStartCustomVector(builder, numElems):
    return builder.StartVector(1, numElems, 1)

def StartCustomVector(builder, numElems: int) -> int:
    return CustomQuantizationStartCustomVector(builder, numElems)

def CustomQuantizationEnd(builder):
    return builder.EndObject()

def End(builder):
    return CustomQuantizationEnd(builder)

try:
    from typing import List
except:
    pass

class CustomQuantizationT(object):

    # CustomQuantizationT
    def __init__(self):
        self.custom = None  # type: List[int]

    @classmethod
    def InitFromBuf(cls, buf, pos):
        customQuantization = CustomQuantization()
        customQuantization.Init(buf, pos)
        return cls.InitFromObj(customQuantization)

    @classmethod
    def InitFromPackedBuf(cls, buf, pos=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, pos)
        return cls.InitFromBuf(buf, pos+n)

    @classmethod
    def InitFromObj(cls, customQuantization):
        x = CustomQuantizationT()
        x._UnPack(customQuantization)
        return x

    # CustomQuantizationT
    def _UnPack(self, customQuantization):
        if customQuantization is None:
            return
        if not customQuantization.CustomIsNone():
            if np is None:
                self.custom = []
                for i in range(customQuantization.CustomLength()):
                    self.custom.append(customQuantization.Custom(i))
            else:
                self.custom = customQuantization.CustomAsNumpy()

    # CustomQuantizationT
    def Pack(self, builder):
        if self.custom is not None:
            if np is not None and type(self.custom) is np.ndarray:
                custom = builder.CreateNumpyVector(self.custom)
            else:
                CustomQuantizationStartCustomVector(builder, len(self.custom))
                for i in reversed(range(len(self.custom))):
                    builder.PrependUint8(self.custom[i])
                custom = builder.EndVector()
        CustomQuantizationStart(builder)
        if self.custom is not None:
            CustomQuantizationAddCustom(builder, custom)
        customQuantization = CustomQuantizationEnd(builder)
        return customQuantization
