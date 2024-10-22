# automatically generated by the FlatBuffers compiler, do not modify

# namespace: tflite

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class SignatureDef(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = SignatureDef()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsSignatureDef(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    @classmethod
    def SignatureDefBufferHasIdentifier(cls, buf, offset, size_prefixed=False):
        return flatbuffers.util.BufferHasIdentifier(buf, offset, b"\x54\x46\x4C\x33", size_prefixed=size_prefixed)

    # SignatureDef
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # SignatureDef
    def Inputs(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            x = self._tab.Vector(o)
            x += flatbuffers.number_types.UOffsetTFlags.py_type(j) * 4
            x = self._tab.Indirect(x)
            from tflite.TensorMap import TensorMap
            obj = TensorMap()
            obj.Init(self._tab.Bytes, x)
            return obj
        return None

    # SignatureDef
    def InputsLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # SignatureDef
    def InputsIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        return o == 0

    # SignatureDef
    def Outputs(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            x = self._tab.Vector(o)
            x += flatbuffers.number_types.UOffsetTFlags.py_type(j) * 4
            x = self._tab.Indirect(x)
            from tflite.TensorMap import TensorMap
            obj = TensorMap()
            obj.Init(self._tab.Bytes, x)
            return obj
        return None

    # SignatureDef
    def OutputsLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # SignatureDef
    def OutputsIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        return o == 0

    # SignatureDef
    def SignatureKey(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # SignatureDef
    def SubgraphIndex(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Uint32Flags, o + self._tab.Pos)
        return 0

def SignatureDefStart(builder):
    builder.StartObject(5)

def Start(builder):
    SignatureDefStart(builder)

def SignatureDefAddInputs(builder, inputs):
    builder.PrependUOffsetTRelativeSlot(0, flatbuffers.number_types.UOffsetTFlags.py_type(inputs), 0)

def AddInputs(builder, inputs):
    SignatureDefAddInputs(builder, inputs)

def SignatureDefStartInputsVector(builder, numElems):
    return builder.StartVector(4, numElems, 4)

def StartInputsVector(builder, numElems: int) -> int:
    return SignatureDefStartInputsVector(builder, numElems)

def SignatureDefAddOutputs(builder, outputs):
    builder.PrependUOffsetTRelativeSlot(1, flatbuffers.number_types.UOffsetTFlags.py_type(outputs), 0)

def AddOutputs(builder, outputs):
    SignatureDefAddOutputs(builder, outputs)

def SignatureDefStartOutputsVector(builder, numElems):
    return builder.StartVector(4, numElems, 4)

def StartOutputsVector(builder, numElems: int) -> int:
    return SignatureDefStartOutputsVector(builder, numElems)

def SignatureDefAddSignatureKey(builder, signatureKey):
    builder.PrependUOffsetTRelativeSlot(2, flatbuffers.number_types.UOffsetTFlags.py_type(signatureKey), 0)

def AddSignatureKey(builder, signatureKey):
    SignatureDefAddSignatureKey(builder, signatureKey)

def SignatureDefAddSubgraphIndex(builder, subgraphIndex):
    builder.PrependUint32Slot(4, subgraphIndex, 0)

def AddSubgraphIndex(builder, subgraphIndex):
    SignatureDefAddSubgraphIndex(builder, subgraphIndex)

def SignatureDefEnd(builder):
    return builder.EndObject()

def End(builder):
    return SignatureDefEnd(builder)

import tflite.TensorMap
try:
    from typing import List
except:
    pass

class SignatureDefT(object):

    # SignatureDefT
    def __init__(self):
        self.inputs = None  # type: List[tflite.TensorMap.TensorMapT]
        self.outputs = None  # type: List[tflite.TensorMap.TensorMapT]
        self.signatureKey = None  # type: str
        self.subgraphIndex = 0  # type: int

    @classmethod
    def InitFromBuf(cls, buf, pos):
        signatureDef = SignatureDef()
        signatureDef.Init(buf, pos)
        return cls.InitFromObj(signatureDef)

    @classmethod
    def InitFromPackedBuf(cls, buf, pos=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, pos)
        return cls.InitFromBuf(buf, pos+n)

    @classmethod
    def InitFromObj(cls, signatureDef):
        x = SignatureDefT()
        x._UnPack(signatureDef)
        return x

    # SignatureDefT
    def _UnPack(self, signatureDef):
        if signatureDef is None:
            return
        if not signatureDef.InputsIsNone():
            self.inputs = []
            for i in range(signatureDef.InputsLength()):
                if signatureDef.Inputs(i) is None:
                    self.inputs.append(None)
                else:
                    tensorMap_ = tflite.TensorMap.TensorMapT.InitFromObj(signatureDef.Inputs(i))
                    self.inputs.append(tensorMap_)
        if not signatureDef.OutputsIsNone():
            self.outputs = []
            for i in range(signatureDef.OutputsLength()):
                if signatureDef.Outputs(i) is None:
                    self.outputs.append(None)
                else:
                    tensorMap_ = tflite.TensorMap.TensorMapT.InitFromObj(signatureDef.Outputs(i))
                    self.outputs.append(tensorMap_)
        self.signatureKey = signatureDef.SignatureKey()
        self.subgraphIndex = signatureDef.SubgraphIndex()

    # SignatureDefT
    def Pack(self, builder):
        if self.inputs is not None:
            inputslist = []
            for i in range(len(self.inputs)):
                inputslist.append(self.inputs[i].Pack(builder))
            SignatureDefStartInputsVector(builder, len(self.inputs))
            for i in reversed(range(len(self.inputs))):
                builder.PrependUOffsetTRelative(inputslist[i])
            inputs = builder.EndVector()
        if self.outputs is not None:
            outputslist = []
            for i in range(len(self.outputs)):
                outputslist.append(self.outputs[i].Pack(builder))
            SignatureDefStartOutputsVector(builder, len(self.outputs))
            for i in reversed(range(len(self.outputs))):
                builder.PrependUOffsetTRelative(outputslist[i])
            outputs = builder.EndVector()
        if self.signatureKey is not None:
            signatureKey = builder.CreateString(self.signatureKey)
        SignatureDefStart(builder)
        if self.inputs is not None:
            SignatureDefAddInputs(builder, inputs)
        if self.outputs is not None:
            SignatureDefAddOutputs(builder, outputs)
        if self.signatureKey is not None:
            SignatureDefAddSignatureKey(builder, signatureKey)
        SignatureDefAddSubgraphIndex(builder, self.subgraphIndex)
        signatureDef = SignatureDefEnd(builder)
        return signatureDef
