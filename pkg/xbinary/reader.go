package xbinary

import (
	"bytes"
	"encoding/binary"
	"io"
)

func Read[T any](r io.Reader, data T) error {
	return binary.Read(r, binary.LittleEndian, data)
}

func ReadString(r io.Reader, data *string, length int) error {
	buf := make([]byte, length)

	if err := binary.Read(r, binary.LittleEndian, buf); err != nil {
		return err
	}

	// Sometimes we cannot fill the entire buffer, so if we find a NUL byte, we can cut off from there.
	if nullIndex := bytes.IndexByte(buf, '\x00'); nullIndex != -1 {
		buf = buf[:nullIndex]
	}

	*data = string(buf)

	return nil
}
