package xbinary

import (
	"encoding/binary"
	"io"
)

func Write[T any](w io.Writer, data T) error {
	return binary.Write(w, binary.LittleEndian, data)
}

func WriteString(w io.Writer, data string, length int) error {
	// Copy up to `length` bytes, no matter if the `data` is longer.
	// That is because the packet structure is fixed and we can't overflow it.
	tmp := make([]byte, length)

	copy(tmp, []byte(data))

	return binary.Write(w, binary.LittleEndian, tmp)
}
